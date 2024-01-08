// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "task_state"))]
    pub struct TaskState;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TaskState;

    ac_task (id) {
        id -> Int4,
        title -> Varchar,
        body -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Varchar,
        state -> TaskState,
    }
}
