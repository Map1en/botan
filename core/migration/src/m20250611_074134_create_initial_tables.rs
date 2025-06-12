use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Users::Username).string())
                    .col(ColumnDef::new(Users::DisplayName).string().not_null())
                    .col(ColumnDef::new(Users::Bio).text().not_null())
                    .col(ColumnDef::new(Users::IsFriend).boolean().not_null())
                    .col(ColumnDef::new(Users::LastLogin).string().not_null())
                    .col(ColumnDef::new(Users::Pronouns).string().not_null())
                    .col(ColumnDef::new(Users::Status).string().not_null())
                    .col(ColumnDef::new(Users::StatusDescription).text().not_null())
                    .col(ColumnDef::new(Users::ProfilePicOverride).text())
                    .col(ColumnDef::new(Users::UserIcon).text())
                    .col(ColumnDef::new(Users::LastApiUpdateAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Users::RawData).json())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Friendships::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Friendships::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Friendships::OwnerUserId).string().not_null())
                    .col(
                        ColumnDef::new(Friendships::FriendUserId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Friendships::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(Friendships::FriendedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Friendships::UnfriendedAt).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-friendships-friend_user_id")
                            .from(Friendships::Table, Friendships::FriendUserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserAttributeHistory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserAttributeHistory::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserAttributeHistory::UserId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserAttributeHistory::AttributeName)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserAttributeHistory::OldValue).text())
                    .col(ColumnDef::new(UserAttributeHistory::NewValue).text())
                    .col(
                        ColumnDef::new(UserAttributeHistory::ChangedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-history-user_id")
                            .from(UserAttributeHistory::Table, UserAttributeHistory::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserLocationHistory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserLocationHistory::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserLocationHistory::UserId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserLocationHistory::Location).string())
                    .col(ColumnDef::new(UserLocationHistory::WorldId).string())
                    .col(
                        ColumnDef::new(UserLocationHistory::RecordedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-location-history-user_id")
                            .from(UserLocationHistory::Table, UserLocationHistory::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserLocationHistory::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserAttributeHistory::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Friendships::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    DisplayName,
    Bio,
    IsFriend,
    LastLogin,
    Pronouns,
    Status,
    StatusDescription,
    ProfilePicOverride,
    UserIcon,
    LastApiUpdateAt,
    RawData,
    CreatedAt,
    UpdatedAt,
}
#[derive(DeriveIden)]
enum Friendships {
    Table,
    Id,
    OwnerUserId,
    FriendUserId,
    IsActive,
    FriendedAt,
    UnfriendedAt,
}
#[derive(DeriveIden)]
enum UserAttributeHistory {
    Table,
    Id,
    UserId,
    AttributeName,
    OldValue,
    NewValue,
    ChangedAt,
}
#[derive(DeriveIden)]
enum UserLocationHistory {
    Table,
    Id,
    UserId,
    Location,
    WorldId,
    RecordedAt,
}
