use crate::client;
use crate::services::event_service;
use anyhow::Result;
use chrono::{DateTime, Local, Utc};
use futures_util::StreamExt;
use reqwest::header::{HeaderValue, USER_AGENT};
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tokio::time::sleep;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, Message},
};
use url::Url;

pub struct PipelineHandler {}

impl PipelineHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn listen(&self, auth_token: &str) -> Result<()> {
        let url_str = format!("wss://pipeline.vrchat.cloud/?authToken={}", auth_token);
        let _url = Url::parse(&url_str)?;

        let mut request = url_str.clone().into_client_request()?;

        let user_agent_value = client::GLOBAL_USER_AGENT.clone();
        request
            .headers_mut()
            .insert(USER_AGENT, HeaderValue::from_str(&user_agent_value)?);
        println!("Connect Pipeline...");
        println!("token: {}", url_str.clone());
        let (ws_stream, response) = connect_async(request).await?;
        println!("Connect Successful HTTP Response: {}", response.status());
        println!("-----------------------------------------");

        let (_write, mut read) = ws_stream.split();

        while let Some(msg) = read.next().await {
            match msg {
                Ok(message) => {
                    if let Err(e) = self.handle_message(message).await {
                        eprintln!("handle_message error: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Connection error: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    async fn handle_message(&self, msg: Message) -> Result<()> {
        if let Message::Text(text) = msg {
            println!("[raw]: {}", text);

            let outer_json: Value = serde_json::from_str(&text)?;

            let event_type = outer_json["type"].as_str().unwrap_or("unknown");
            let content_value = &outer_json["content"];

            let final_content = if content_value.is_string() {
                let content_str = content_value.as_str().unwrap();
                serde_json::from_str(content_str)?
            } else {
                content_value.clone()
            };

            println!("\n[event type]: {}", event_type);
            println!("[event content]:\n{:#?}", final_content);
            println!("Received at: {:?}", Local::now().format("%H:%M:%S"));

            if let Err(e) = event_service::process_websocket_event(event_type, &final_content).await
            {
                log::error!("Failed to process event {}: {}", event_type, e);
            } else {
                log::info!("Event {} processed and saved to database", event_type);
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct PipelineStatus {
    pub connected: bool,
    pub last_message_time: Option<DateTime<Utc>>,
    pub reconnect_count: u32,
}

pub struct PipelineManager {
    auth_token: String,
    status: Arc<RwLock<PipelineStatus>>,
    shutdown_sender: Option<mpsc::UnboundedSender<()>>,
}

impl PipelineManager {
    pub fn new(auth_token: String) -> Self {
        Self {
            auth_token,
            status: Arc::new(RwLock::new(PipelineStatus {
                connected: false,
                last_message_time: None,
                reconnect_count: 0,
            })),
            shutdown_sender: None,
        }
    }

    pub async fn start(&mut self) {
        let (shutdown_tx, mut shutdown_rx) = mpsc::unbounded_channel();
        self.shutdown_sender = Some(shutdown_tx);

        let auth_token = self.auth_token.clone();
        let status = self.status.clone();

        tokio::spawn(async move {
            let mut reconnect_count = 0;
            let max_retries = 50;

            loop {
                if shutdown_rx.try_recv().is_ok() {
                    println!("Pipeline manager received shutdown signal");
                    break;
                }

                println!(
                    "Starting pipeline connection attempt {}...",
                    reconnect_count + 1
                );

                {
                    let mut status_guard = status.write().await;
                    status_guard.connected = false;
                    status_guard.reconnect_count = reconnect_count;
                }

                let handler = PipelineHandler::new();
                match handler.listen(&auth_token).await {
                    Ok(_) => {
                        println!("Pipeline connection ended normally");
                        reconnect_count = 0;
                        println!("Reconnecting pipeline in 5 seconds...");
                        sleep(Duration::from_secs(5)).await;
                    }
                    Err(e) => {
                        reconnect_count += 1;
                        eprintln!(
                            "Pipeline connection failed (attempt {}/{}): {}",
                            reconnect_count, max_retries, e
                        );

                        if reconnect_count >= max_retries {
                            eprintln!("Pipeline max retries exceeded, but this is a service, so we'll keep trying...");
                            reconnect_count = 0;
                        }

                        let delay = Duration::from_secs(2u64.pow(reconnect_count.min(5)));
                        println!("Reconnecting pipeline in {:?}...", delay);
                        sleep(delay).await;
                    }
                }
            }

            println!("Pipeline manager shut down");
        });

        println!("Pipeline manager started in background");
    }

    pub async fn get_status(&self) -> PipelineStatus {
        self.status.read().await.clone()
    }

    pub async fn shutdown(&mut self) {
        if let Some(sender) = &self.shutdown_sender {
            let _ = sender.send(());
        }
    }
}
