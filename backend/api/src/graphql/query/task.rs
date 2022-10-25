use crate::authority::Claims;
use async_graphql::{Context, Object, Result};
use repository::{async_graphql, db::Database, task};

#[derive(Default)]
pub struct TaskQuery;

#[Object]
impl TaskQuery {
    async fn tasks(&self, ctx: &Context<'_>, user_id: i32) -> Result<Vec<task::Model>> {
        let claims = ctx.data_opt::<Claims>();
        println!("claims: {:?}", claims);
        if let Some(claims) = claims {
            println!("claims: {:?}", claims);
        }
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        Ok(task::Entity::find_by_user_id(user_id)
            .all(conn)
            .await
            .map_err(|e| e.to_string())?)
    }
}
