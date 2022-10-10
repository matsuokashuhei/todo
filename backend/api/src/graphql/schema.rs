use crate::graphql::{mutation::Mutation, query::Query};
use async_graphql::{EmptySubscription, Schema};
use repository::db;

// use crate::db::Database;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema(database: db::Database) -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(database)
        .finish()
}
