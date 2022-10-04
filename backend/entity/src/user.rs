use crate::task;
use async_graphql::*;
use db::Database;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "users")]
#[graphql(complex)]
#[graphql(concrete(name = "User", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    // pub tasks: Vec<task::Model>, // pub task: task::Model, // pub tasks: [task::Model],
}

#[ComplexObject]
impl Model {
    async fn tasks(&self, ctx: &Context<'_>) -> Result<Vec<task::Model>> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        Ok(self
            .find_related(task::Entity)
            .all(conn)
            .await
            .map_err(|err| err.to_string())?)
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Task,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Task => Entity::has_many(super::task::Entity).into(),
        }
    }
}

impl Related<super::task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

// impl Entity {
//     pub fn tasks() -> {
//         Self::find_related(Task).all(db);
//     }
// }

impl ActiveModelBehavior for ActiveModel {}
