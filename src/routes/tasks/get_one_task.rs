use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use sea_orm::{
    sea_query::tests_cfg::Task, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use crate::{
    database::{
        tasks::{self, Entity as Tasks},
        users::Model as UserModel,
    },
    queries,
    routes::tasks::ResponseTask,
    utils::app_error::AppError,
};

use super::ResponseDataTask;

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataTask>, AppError> {
    dbg!(task_id);

    let task = queries::task_queries::find_task_by_id(task_id, &user, &db).await?;

    Ok(Json(ResponseDataTask {
        data: ResponseTask {
            id: task.id as i64,
            title: task.title,
            description: task.description,
            priority: task.priority,
            completed_at: task
                .completed_at
                .map(|completed_at| completed_at.to_string()),
        },
    }))
}
