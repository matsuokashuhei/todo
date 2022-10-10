use crate::authority;
use async_graphql::{Context, Object, Result};
use repository::{async_graphql, db::Database, user};
use sea_orm::EntityTrait;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<user::Model>> {
        let claims = ctx.data::<authority::Claims>().unwrap();
        println!("claims: {:?}", claims);
        println!("sub: {:?}", claims.sub);
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        Ok(user::Entity::find()
            .all(conn)
            .await
            .map_err(|e| e.to_string())?)
    }
    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<Option<user::Model>> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        Ok(user::Entity::find_by_id(id).one(conn).await?)
    }
}

// #[ComplexObject]
// impl user::Model {
//     pub async fn tasks(&self, ctx: &Context<'_>) -> Result<Vec<task::Model>> {
//         let db = ctx.data::<Database>().unwrap();
//         let conn = db.get_connection();
//         Ok(self.find_related(Tasks).all(db).await)
//     }
// }
