use repository::async_graphql;
pub mod user;
pub use user::UserMutation;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(UserMutation);
