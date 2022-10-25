use crate::{authority::Claims, graphql::object::user::User};
use async_graphql::{Context, Object, Result};
use aws_sdk_cognitoidentityprovider::Client;
use repository::{async_graphql, db::Database};
use sea_orm::{DbErr, EntityTrait};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn user(&self, ctx: &Context<'_>) -> Result<User> {
        let claims = ctx.data_opt::<Claims>();
        if let Some(claims) = claims {
            Ok(User {
                sub: claims.sub.to_owned(),
            })
        } else {
            Err(async_graphql::Error::new("Unauthorized"))
        }
    }
    // async fn users(&self, ctx: &Context<'_>) -> Result<Vec<user::Model>> {
    //     let claims = ctx.data_opt::<Claims>();
    //     println!("claims: {:?}", claims);
    //     if let Some(claims) = claims {
    //         println!("claims: {:?}", claims);
    //     }
    //     let db = ctx.data::<Database>().unwrap();
    //     let conn = db.get_connection();
    //     Ok(user::Entity::find()
    //         .all(conn)
    //         .await
    //         .map_err(|e| e.to_string())?)
    // }
    // async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<user::Model> {
    //     let db = ctx.data::<Database>().unwrap();
    //     let conn = db.get_connection();
    //     let user = user::Entity::find_by_id(id)
    //         .one(conn)
    //         .await
    //         .map_err(|e| e.to_string())?;
    //     match user {
    //         Some(user) => Ok(user),
    //         None => Err(async_graphql::Error::new(
    //             DbErr::RecordNotFound(id.to_string()).to_string(),
    //         )),
    //     }
    // }
}
