use crate::db::Database;
use crate::user;
use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "tasks")]
#[graphql(complex, name = "Task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub description: String,
    pub user_id: i32,
}

#[ComplexObject]
impl Model {
    pub async fn user(&self, ctx: &Context<'_>) -> Result<user::Model> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        Ok(self
            .find_related(user::Entity)
            .one(conn)
            .await
            .map_err(|e| e.to_string())
            .unwrap()
            .unwrap())
    }
}

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

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_user_id(user_id: i32) -> Select<Entity> {
        Self::find().filter(Column::UserId.eq(user_id))
    }
}
