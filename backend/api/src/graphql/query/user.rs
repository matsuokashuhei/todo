use async_graphql::{Context, Object, Result};
use entity::{async_graphql, user::Model as User};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn customers(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        Ok(vec![User {
            id: 1,
            name: "TEST".to_owned(),
        }])
    }
}
