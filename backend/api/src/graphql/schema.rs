use crate::graphql::{mutation::Mutation, query::Query};
use async_graphql::{EmptySubscription, Schema};
use repository::db::Database;
// use crate::db::Database;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema() -> AppSchema {
    let db = Database::new().await;
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish()
}
