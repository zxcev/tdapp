use axum::{
    extract::State,
    http::StatusCode,
    Extension, Json,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TryIntoModel};

use crate::{
    database::{
        tasks,
        users::{self, Model as UserModel},
    },
    queries,
    utils::app_error::AppError,
};

use super::{create_task_extractor::ValidateCreateTask, ResponseDataTask, ResponseTask};

pub async fn create_task(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
    task: ValidateCreateTask, // Json(request_task): Json<RequestTask>,
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    // ) -> Result<impl IntoResponse, AppError> {
    let task = queries::task_queries::create_task(task, &user, &db).await?;

    let response = ResponseTask {
        id: task.id as i64,
        title: task.title,
        description: task.description,
        priority: task.priority,
        completed_at: task.completed_at.map(|time| time.to_string()),
    };

    Ok((
        StatusCode::CREATED,
        Json(ResponseDataTask { data: response }),
    ))
}
