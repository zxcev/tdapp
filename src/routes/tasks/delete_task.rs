use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::{
    database::{tasks, users},
    queries::task_queries,
    utils::app_error::AppError,
};

pub async fn soft_delete_task(
    Path(task_id): Path<i32>,
    Extension(user): Extension<users::Model>,
    State(db): State<DatabaseConnection>,
) -> Result<(), AppError> {
    let task = task_queries::find_task_by_id(task_id, &user, &db).await?;

    let mut task = task.into_active_model();
    let now = Utc::now();
    task.deleted_at = Set(Some(now.into()));
    task_queries::save_active_task(task, &db).await?;

    Ok(())
}
