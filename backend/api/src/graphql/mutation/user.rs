use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject};
use entity::user;

#[derive(InputObject)]
pub struct CreateUserInput {
    pub name: String,
}

// impl CreateUserInput {
//     fn into_model_with_arbitray_id(self) -> user::Model {
//         user::Model {
//             id: 0,
//             name: self.name,
//         }
//     }
// }

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> Result<user::Model> {
        Ok(user::Model {
            id: 1,
            name: input.name,
        })
    }
}
