use anyhow::Result;
use futures_util::StreamExt;
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

pub struct PipelineHandler {}

impl PipelineHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn listen(&self, auth_token: &str) -> Result<()> {
        let url_str = format!("wss://pipeline.vrchat.cloud/?authToken={}", auth_token);
        let _url = Url::parse(&url_str)?;

        println!("Connect Pipeline...");
        println!("token: {}", url_str);
        let (ws_stream, response) = connect_async(url_str).await?;
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
                    eprintln!("need reconnect: {}", e);
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
        }
        Ok(())
    }
}
