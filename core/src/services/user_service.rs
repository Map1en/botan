use crate::database::get_db_connection;
use crate::entities::{prelude::*, users};
use sea_orm::sea_query::OnConflict;
use sea_orm::*;
use vrchatapi::models::User;

pub async fn upsert_user(api_user: &User) -> Result<users::Model, DbErr> {
    let db = get_db_connection()
        .await
        .ok_or_else(|| DbErr::Custom("Database connection not available".to_string()))?;

    let user_model = users::ActiveModel::from(api_user.clone());

    let insert_result = Users::insert(user_model)
        .on_conflict(
            OnConflict::column(users::Column::Id)
                .update_columns([
                    users::Column::Username,
                    users::Column::DisplayName,
                    users::Column::Bio,
                    users::Column::Status,
                    users::Column::StatusDescription,
                ])
                .to_owned(),
        )
        .exec(&db)
        .await?;

    log::info!("User upsert result - ID: {}", insert_result.last_insert_id);

    Users::find_by_id(&api_user.id)
        .one(&db)
        .await?
        .ok_or_else(|| DbErr::Custom("Failed to retrieve user after upsert".to_string()))
}
