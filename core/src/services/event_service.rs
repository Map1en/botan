use crate::conversions::*;
use crate::services::user_service;
use anyhow::Result;
use serde_json::Value;

pub async fn process_websocket_event(event_type: &str, content: &Value) -> Result<()> {
    log::info!("Processing event: {}", event_type);

    match event_type {
        "friend-add" => {
            if let Ok(event) = serde_json::from_value::<FriendAddEvent>(content.clone()) {
                process_friend_add_event(event).await?;
            }
        }
        "friend-delete" => {
            if let Ok(event) = serde_json::from_value::<FriendDeleteEvent>(content.clone()) {
                process_friend_delete_event(event).await?;
            }
        }
        "friend-online" => {
            if let Ok(event) = serde_json::from_value::<FriendOnlineEvent>(content.clone()) {
                process_friend_online_event(event).await?;
            }
        }
        "friend-active" => {
            if let Ok(event) = serde_json::from_value::<FriendActiveEvent>(content.clone()) {
                process_friend_active_event(event).await?;
            }
        }
        "friend-offline" => {
            if let Ok(event) = serde_json::from_value::<FriendOfflineEvent>(content.clone()) {
                process_friend_offline_event(event).await?;
            }
        }
        "friend-update" => {
            if let Ok(event) = serde_json::from_value::<FriendUpdateEvent>(content.clone()) {
                process_friend_update_event(event).await?;
            }
        }
        "friend-location" => {
            if let Ok(event) = serde_json::from_value::<FriendLocationEvent>(content.clone()) {
                process_friend_location_event(event).await?;
            }
        }
        _ => {
            log::warn!("Unknown event type: {}", event_type);
        }
    }

    Ok(())
}

async fn process_friend_add_event(event: FriendAddEvent) -> Result<()> {
    log::info!("Friend added: {}", event.user.display_name);

    if let Err(e) = user_service::upsert_user(&event.user).await {
        log::error!("Failed to upsert user: {}", e);
    }

    // more methods?

    Ok(())
}

async fn process_friend_delete_event(event: FriendDeleteEvent) -> Result<()> {
    log::info!("Friend deleted: {}", event.user.display_name);

    if let Err(e) = user_service::upsert_user(&event.user).await {
        log::error!("Failed to upsert user: {}", e);
    }

    Ok(())
}

async fn process_friend_online_event(event: FriendOnlineEvent) -> Result<()> {
    log::info!(
        "Friend online: {} at {}",
        event.user.display_name,
        event.location.as_deref().unwrap_or("Unknown")
    );

    if let Err(e) = user_service::upsert_user(&event.user).await {
        log::error!("Failed to upsert user: {}", e);
    }

    Ok(())
}

async fn process_friend_active_event(event: FriendActiveEvent) -> Result<()> {
    log::info!("Friend active: {}", event.user.display_name);

    if let Err(e) = user_service::upsert_user(&event.user).await {
        log::error!("Failed to upsert user: {}", e);
    }

    Ok(())
}

async fn process_friend_offline_event(event: FriendOfflineEvent) -> Result<()> {
    log::info!("Friend offline: {}", event.user_id);

    // only offline time? pending??

    Ok(())
}

async fn process_friend_update_event(event: FriendUpdateEvent) -> Result<()> {
    log::info!("Friend updated: {}", event.user.display_name);

    if let Err(e) = user_service::upsert_user(&event.user).await {
        log::error!("Failed to upsert user: {}", e);
    }

    Ok(())
}

async fn process_friend_location_event(event: FriendLocationEvent) -> Result<()> {
    log::info!(
        "Friend location: {} at {}",
        event.user.display_name,
        event.location.as_deref().unwrap_or("Unknown")
    );

    if let Err(e) = user_service::upsert_user(&event.user).await {
        log::error!("Failed to upsert user: {}", e);
    }

    Ok(())
}
