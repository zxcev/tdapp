use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    database::{tasks, users},
    queries::task_queries,
    utils::app_error::AppError,
};

use super::{ResponseDataTask, ResponseDataTasks, ResponseTask};

pub async fn get_all_tasks(
    Extension(user): Extension<users::Model>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataTasks>, AppError> {
    let tasks = task_queries::get_all_tasks(user.id, &db, false).await?;

    let tasks = tasks
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id as i64,
            title: db_task.title,
            description: db_task.description,
            priority: db_task.priority,
            completed_at: db_task
                .completed_at
                .map(|completed_at| completed_at.to_string()),
        })
        .collect::<Vec<ResponseTask>>();

    Ok(Json(ResponseDataTasks { data: tasks }))
}
