pub mod user;
pub use user::UserQuery;
pub mod task;
pub use task::TaskQuery;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(UserQuery, TaskQuery);
