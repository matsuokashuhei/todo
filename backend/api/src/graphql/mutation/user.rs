use crate::db::Database;
use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject};
use entity::user;
use sea_orm::{DbErr, EntityTrait, Set};

#[derive(InputObject)]
pub struct CreateUserInput {
    pub name: String,
}

impl CreateUserInput {
    fn into_model_with_arbitray_id(self) -> user::Model {
        user::Model {
            id: 0,
            name: self.name,
        }
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> Result<user::Model, DbErr> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        let active_model = user::ActiveModel {
            name: Set(input.name.to_owned()),
            ..Default::default()
        };
        let res = user::Entity::insert(active_model).exec(conn).await;
        match res {
            Ok(res) => Ok(user::Model {
                id: res.last_insert_id,
                name: input.name,
            }),
            Err(e) => Err(e),
        }
    }
}
