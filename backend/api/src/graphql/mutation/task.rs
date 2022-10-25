use async_graphql::{Context, Object, Result};
use repository::{
    async_graphql::{self, InputObject},
    db::Database,
    task,
};
use sea_orm::{EntityTrait, Set};

#[derive(InputObject)]
pub struct CreateTaskInput {
    pub user_id: i32,
    pub title: String,
    pub description: String,
}

impl CreateTaskInput {
    fn into_model_with_arbitray_id(self) -> task::Model {
        task::Model {
            id: 0,
            user_id: self.user_id,
            title: self.title,
            description: self.description,
        }
    }
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
        let active_model = task::ActiveModel {
            user_id: Set(input.user_id.to_owned()),
            title: Set(input.title.to_owned()),
            description: Set(input.description.to_owned()),
            ..Default::default()
        };
        let result = task::Entity::insert(active_model)
            .exec(conn)
            .await
            .map_err(|e| e.to_string())?;
        Ok(task::Model {
            id: result.last_insert_id,
            user_id: input.user_id,
            title: input.title,
            description: input.description,
        })
    }
}
