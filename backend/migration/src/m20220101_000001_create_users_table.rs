use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectionTrait, Statement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // manager
        //     .create_table(
        //         Table::create()
        //             .table(Users::Table)
        //             .if_not_exists()
        //             .col(
        //                 ColumnDef::new(Users::Id)
        //                     .integer()
        //                     .not_null()
        //                     .auto_increment()
        //                     .primary_key(),
        //             )
        //             .col(ColumnDef::new(Users::Name).string().not_null())
        //             // .col(ColumnDef::new(Users::CreatedAt).timestamp().not_null())
        //             // .col(ColumnDef::new(Users::UpdatedAt).timestamp().not_null())
        //             .to_owned(),
        //     )
        //     .await
        let sql = r#"
        CREATE TABLE `users` (
            `id` int NOT NULL AUTO_INCREMENT PRIMARY KEY,
            `name` varchar(255) NOT NULL,
            `created_at` timestamp DEFAULT CURRENT_TIMESTAMP,
            `updated_at` timestamp DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
        )
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // manager
        //     .drop_table(Table::drop().table(Users::Table).to_owned())
        //     .await
        let sql = "DROP TABLE IF EXISTS `users`";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Users {
    // Table,
    // Id,
    // Name,
    // CreatedAt,
    // UpdatedAt,
}
