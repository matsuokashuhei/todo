use sea_orm_migration::{prelude::*, sea_orm::{ConnectionTrait, Statement}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        ALTER TABLE `tasks`
        DROP FOREIGN KEY `tasks_ibfk_1`;
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())?;
        let sql = r#"
        ALTER TABLE `tasks`
        DROP COLUMN `user_id`
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        ALTER TABLE `tasks`
        ADD `user_id` int NOT NULL
        ADD CONSTRAINT `user_id` FOREIGN KEY (`user_id`)
        REFERENCES `users`(`id`)
        AFTER `id`
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
