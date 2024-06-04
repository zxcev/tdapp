use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::{
    database::{tasks, users},
    queries::{self, task_queries},
    utils::app_error::AppError,
};

use super::{RequestTask, RequestUpdateTask};

pub async fn mark_completed(
    Path(task_id): Path<i32>,
    Extension(user): Extension<users::Model>,
    State(db): State<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut task = task_queries::find_task_by_id(task_id, &user, &db)
        .await?
        .into_active_model();

    // task를 mut로 설정해야 Set(Some(now.into())) 정상 실행 가능.
    // 구조체가 mut일 경우, 구조체 내의 모든 direct field가 mut가 되기 때문

    let now = Utc::now();
    task.completed_at = Set(Some(now.into()));
    task_queries::save_active_task(task, &db).await?;

    Ok(())
}

pub async fn mark_uncompleted(
    Path(task_id): Path<i32>,
    Extension(user): Extension<users::Model>,
    State(db): State<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut task = task_queries::find_task_by_id(task_id, &user, &db)
        .await?
        .into_active_model();

    task.completed_at = Set(None);
    task_queries::save_active_task(task, &db).await?;

    Ok(())
}

pub async fn update_task(
    Path(task_id): Path<i32>,
    Extension(user): Extension<users::Model>,
    State(db): State<DatabaseConnection>,
    Json(request_task): Json<RequestUpdateTask>,
) -> Result<(), AppError> {
    let mut task = task_queries::find_task_by_id(task_id, &user, &db)
        .await?
        .into_active_model();

    // task를 mut로 설정해야 Set(Some(now.into())) 정상 실행 가능.
    // 구조체가 mut일 경우, 구조체 내의 모든 direct field가 mut가 되기 때문
    // let mut task = if let Some(task) = task {
    //     dbg!("ok");
    //     task.into_active_model()
    // } else {
    //     dbg!("err");
    //     return Err(AppError::new(StatusCode::NOT_FOUND, "not found"));
    // };

    if let Some(priority) = request_task.priority {
        task.priority = Set(priority);
    }
    if let Some(title) = request_task.title {
        task.title = Set(title);
    }
    if let Some(completed_at) = request_task.completed_at {
        task.completed_at = Set(completed_at);
    }
    if let Some(description) = request_task.description {
        task.description = Set(description);
    }
    task_queries::save_active_task(task, &db).await?;

    Ok(())
}
