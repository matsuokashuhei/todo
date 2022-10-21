use async_graphql::{Context, Object, Result};
use repository::{
    async_graphql::{self, InputObject},
    db::Database,
    user,
};
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
    ) -> Result<user::Model> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        let active_model = user::ActiveModel {
            name: Set(input.name.to_owned()),
            ..Default::default()
        };
        let result = user::Entity::insert(active_model)
            .exec(conn)
            .await
            .map_err(|e| e.to_string())?;
        Ok(user::Model {
            id: result.last_insert_id,
            name: input.name,
        })
    }
}
