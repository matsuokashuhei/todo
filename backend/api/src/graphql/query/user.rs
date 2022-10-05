use crate::db::Database;
use async_graphql::{Context, Object, Result};
use entity::{async_graphql, user};
use sea_orm::EntityTrait;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<user::Model>> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        Ok(user::Entity::find()
            .all(conn)
            .await
            .map_err(|e| e.to_string())?)
    }
}
