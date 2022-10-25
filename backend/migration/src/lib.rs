pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users_table;
mod m20221002_115406_create_tasks_table;
mod m20221025_092604_add_user_sub_to_tasks;
mod m20221025_093208_remove_user_id_from_tasks;
mod m20221025_101115_drop_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users_table::Migration),
            Box::new(m20221002_115406_create_tasks_table::Migration),
            Box::new(m20221025_092604_add_user_sub_to_tasks::Migration),
            Box::new(m20221025_093208_remove_user_id_from_tasks::Migration),
            Box::new(m20221025_101115_drop_users::Migration),
        ]
    }
}
