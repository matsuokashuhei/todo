use repository::async_graphql;
pub mod user;
pub use user::UserMutation;
pub mod task;
pub use task::TaskMutation;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(UserMutation, TaskMutation);
