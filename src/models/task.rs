use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use crate::api::api_errors::ApiError;
use crate::pgdb;
use crate::schema::ac_task;

#[derive(Serialize, Deserialize, DbEnum, Debug, Eq, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::TaskState"]
pub enum TaskState {
    NotStarted,
    InProgress,
    Completed,
    Paused,
    Failed,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "ac_task"]
pub struct TaskMsg {
    pub title: String,
    pub body: Option<String>,
    pub type_: String,
    pub state: TaskState,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = ac_task)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub title: String,
    pub body: Option<String>,
    pub type_: String,
    pub state: TaskState,
}

impl Task {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let mut conn = pgdb::connection()?;

        let tasks = ac_task::table.load::<Task>(&mut conn)?;

        Ok(tasks)
    }

    pub fn find(id: i32) -> Result<Self, ApiError> {
        let mut conn = pgdb::connection()?;

        let task = ac_task::table.filter(ac_task::id.eq(id)).first(&mut conn)?;

        Ok(task)
    }

    pub fn create(task_msg: TaskMsg) -> Result<Self, ApiError> {
        let mut conn = pgdb::connection()?;

        let task = Task::from(task_msg);
        let task = diesel::insert_into(ac_task::table)
            .values(task)
            .get_result(&mut conn)?;

        Ok(task)
    }

    pub fn update(id: i32, task_msg: TaskMsg) -> Result<Self, ApiError> {
        let mut conn = pgdb::connection()?;

        let task = diesel::update(ac_task::table)
            .filter(ac_task::id.eq(id))
            .set(Task::from(task_msg))
            .get_result(&mut conn)?;

        Ok(task)
    }

    pub fn delete(id: i32) -> Result<usize, ApiError> {
        let mut conn = pgdb::connection()?;

        let res = diesel::delete(ac_task::table.filter(ac_task::id.eq(id))).execute(&mut conn)?;

        Ok(res)
    }
}

impl From<TaskMsg> for Task {
    fn from(task_msg: TaskMsg) -> Self {
        Task {
            id: None,
            title: task_msg.title,
            body: task_msg.body,
            type_: task_msg.type_,
            state: task_msg.state,
        }
    }
}
