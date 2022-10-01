use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, ID};

#[derive(Clone)]
pub struct User {
    id: ID,
    name: String,
}

#[Object]
impl User {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn name(&self) -> &str {
        &self.name
    }
}

pub type TodoSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> TodoSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users(&self, _ctx: &Context<'_>) -> Vec<User> {
        vec![User {
            id: async_graphql::ID::from("1"),
            name: "name".to_string(),
        }]
    }
}
