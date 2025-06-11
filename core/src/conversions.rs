use vrchatapi::models::User;

use crate::entities::users;
use sea_orm::entity::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "content")]
#[serde(rename_all = "kebab-case")]
pub enum WebsocketEvent {
    FriendAdd(FriendAddEvent),
    FriendDelete(FriendDeleteEvent),
    FriendOnline(FriendOnlineEvent),
    FriendActive(FriendActiveEvent),
    FriendOffline(FriendOfflineEvent),
    FriendUpdate(FriendUpdateEvent),
    FriendLocation(FriendLocationEvent),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FriendAddEvent {
    pub user_id: String,
    pub user: User,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FriendDeleteEvent {
    pub user: User,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FriendOnlineEvent {
    pub user_id: String,
    pub platform: Option<String>,
    pub location: Option<String>,
    pub can_request_invite: bool,
    pub user: User,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FriendActiveEvent {
    pub user_id: String,
    pub platform: Option<String>,
    pub user: User,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FriendOfflineEvent {
    pub user_id: String,
    pub platform: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FriendUpdateEvent {
    pub user_id: String,
    pub user: User,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FriendLocationEvent {
    pub user_id: String,
    pub location: Option<String>,
    pub traveling_to_location: Option<String>,
    pub world_id: String,
    pub can_request_invite: bool,
    pub user: User,
}

impl From<User> for users::ActiveModel {
    fn from(api_user: User) -> Self {
        Self {
            id: Set(api_user.id),
            username: Set(api_user.username),
            display_name: Set(api_user.display_name),
            bio: Set(api_user.bio),
            status: Set(api_user.status.to_string()),
            status_description: Set(api_user.status_description),

            ..Default::default()
        }
    }
}
