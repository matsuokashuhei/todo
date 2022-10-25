use crate::authority::Claims;
use async_graphql::{Context, Object, Result};
use repository::{async_graphql, db::Database, user};
use sea_orm::{DbErr, EntityTrait};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<user::Model>> {
        let claims = ctx.data_opt::<Claims>();
        println!("claims: {:?}", claims);
        if let Some(claims) = claims {
            println!("claims: {:?}", claims);
        }
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        Ok(user::Entity::find()
            .all(conn)
            .await
            .map_err(|e| e.to_string())?)
    }
    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<user::Model> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        let user = user::Entity::find_by_id(id)
            .one(conn)
            .await
            .map_err(|e| e.to_string())?;
        match user {
            Some(user) => Ok(user),
            None => Err(async_graphql::Error::new(
                DbErr::RecordNotFound(id.to_string()).to_string(),
            )),
        }
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
