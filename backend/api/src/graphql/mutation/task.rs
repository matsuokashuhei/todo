use async_graphql::{Context, Object, Result};
use repository::{
    async_graphql::{self, InputObject},
    db::Database,
    task,
};
use sea_orm::{ActiveValue, EntityTrait};

#[derive(InputObject)]
pub struct CreateTaskInput {
    pub user_id: i32,
    pub title: String,
    pub description: String,
}

impl CreateTaskInput {
    fn into_active_model(self) -> task::ActiveModel {
        task::ActiveModel {
            user_id: ActiveValue::Set(self.user_id.to_owned()),
            title: ActiveValue::Set(self.title.to_owned()),
            description: ActiveValue::Set(self.description.to_owned()),
            ..Default::default()
        }
    }
}

#[derive(InputObject)]
pub struct UpdateTaskInput {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
}

impl UpdateTaskInput {
    fn into_active_model(self) -> task::ActiveModel {
        task::ActiveModel {
            id: ActiveValue::Set(self.id),
            title: self
                .title
                .map_or(ActiveValue::NotSet, |title| ActiveValue::Set(title)),
            description: self.description.map_or(ActiveValue::NotSet, |description| {
                ActiveValue::Set(description)
            }),
            ..Default::default() // ..Default::default(),
        }
    }
}

#[derive(InputObject)]
pub struct DeleteTaskInput {
    pub id: i32,
}

#[derive(Default)]
pub struct TaskMutation;

#[Object]
impl TaskMutation {
    pub async fn create_task(
        &self,
        ctx: &Context<'_>,
        input: CreateTaskInput,
    ) -> Result<task::Model> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        let active_model = input.into_active_model();
        let result = task::Entity::insert(active_model)
            .exec(conn)
            .await
            .map_err(|e| e.to_string())?;
        Ok(task::Entity::find_by_id(result.last_insert_id)
            .one(conn)
            .await
            .map_err(|e| e.to_string())
            .unwrap()
            .unwrap())
    }
    pub async fn update_task(
        &self,
        ctx: &Context<'_>,
        input: UpdateTaskInput,
    ) -> Result<task::Model> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        let active_model = input.into_active_model();
        let result = task::Entity::update(active_model)
            .exec(conn)
            .await
            .map_err(|e| e.to_string())?;
        Ok(task::Entity::find_by_id(result.id)
            .one(conn)
            .await
            .map_err(|e| e.to_string())
            .unwrap()
            .unwrap())
    }
    pub async fn delete_task(
        &self,
        ctx: &Context<'_>,
        input: DeleteTaskInput,
    ) -> Result<task::Model> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        let task = task::Entity::find_by_id(input.id)
            .one(conn)
            .await
            .map_err(|e| e.to_string())
            .unwrap()
            .unwrap();
        let _ = task::Entity::delete_by_id(input.id)
            .exec(conn)
            .await
            .map_err(|e| e.to_string())?;
        Ok(task)
    }
}
