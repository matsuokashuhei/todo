use async_graphql::{futures_util::TryFutureExt, Context, Object, Result};
use db::Database;
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
    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<Option<user::Model>> {
        // TODO: Remove Option
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        Ok(user::Entity::find_by_id(id)
            .one(conn)
            .await
            .map_err(|e| e.to_string())?)
    }
}
