use async_graphql::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::Database;
use crate::task;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "users")]
#[graphql(complex, concrete(name = "User", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

// #[ComplexObject]
// impl Model {
//     pub async fn tasks(&self, ctx: &Context<'_>) -> Result<Vec<task::Model>> {
//         // let db = Database::new().await;
//         let db = ctx.data::<Database>().unwrap();
//         let conn = db.get_connection();
//         Ok(self
//             .find_related(task::Entity)
//             .all(conn)
//             .await
//             .map_err(|e| e.to_string())?)
//     }
// }

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

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn tasks() -> Select<Entity> {
        // Self::find_related(task::Entity)
        Self::find().filter(Column::Id.eq(1))
    }
}
