use crate::authority::Claims;
use async_graphql::{Context, Object, Result};
use repository::{async_graphql, db::Database, task};

#[derive(Default)]
pub struct TaskQuery;

#[Object]
impl TaskQuery {
    async fn tasks(&self, ctx: &Context<'_>) -> Result<Vec<task::Model>> {
        let claims = ctx.data_opt::<Claims>();
        if let Some(claims) = claims {
            let db = ctx.data::<Database>().unwrap();
            let conn = db.get_connection();
            Ok(task::Entity::find_by_user_sub(claims.sub.to_owned())
                .all(conn)
                .await
                .map_err(|e| e.to_string())?)
        } else {
            Err(async_graphql::Error::new("Unauthorized"))
        }
    }
}
