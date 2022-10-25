use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, SimpleObject)]
pub struct User {
    pub sub: String,
}
