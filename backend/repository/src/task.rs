use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "tasks")]
#[graphql(concrete(name = "Task", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub description: String,
    pub user_id: i32,
}

// #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
// pub enum Column {
//     Id,
//     Title,
//     Description,
//     UserId,
// }

// impl ColumnTrait for Column {
//     fn def(&self) -> ColumnDef {
//         match self {
//             Self::Id => ColumnType::Integer.def(),
//             Self::Title => ColumnType::String(()).def(),
//             Self::Description => ColumnType::String(()).def(),
//             Self::UserId => ColumnType::Integer.def(),
//         }
//     }
// }

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::UserId)
                .to(super::user::Column::Id)
                .into(),
        }
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
// #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, SimpleObject)]
// #[sea_orm(table_name = "tasks")]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     pub id: i32,
//     pub title: String,
//     pub description: String,
// }

impl ActiveModelBehavior for ActiveModel {}
