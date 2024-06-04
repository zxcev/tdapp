use core::task;

use axum::http::StatusCode;
use sea_orm::{
    sea_query::tests_cfg::Task, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, Set, TryIntoModel,
};

use crate::{
    database::{tasks, users},
    routes::tasks::create_task_extractor::ValidateCreateTask,
    utils::app_error::AppError,
};

pub async fn find_task_by_id(
    id: i32,
    user: &users::Model,
    db: &DatabaseConnection,
) -> Result<tasks::Model, AppError> {
    let task = tasks::Entity::find_by_id(id)
        .filter(tasks::Column::UserId.eq(user.id))
        .one(db)
        .await
        .map_err(|err| {
            eprintln!("Error find task: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error finding task.")
        })?;

    task.ok_or_else(|| {
        eprintln!("Could not find task by id");
        AppError::new(StatusCode::NOT_FOUND, "not found")
    })
}

pub async fn create_task(
    task_data: ValidateCreateTask,
    user: &users::Model,
    db: &DatabaseConnection,
) -> Result<tasks::Model, AppError> {
    let new_task = tasks::ActiveModel {
        priority: Set(task_data.priority),
        title: Set(task_data.title.unwrap()),
        description: Set(task_data.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    save_active_task(new_task, db).await
}

pub async fn save_active_task(
    task: tasks::ActiveModel,
    db: &DatabaseConnection,
) -> Result<tasks::Model, AppError> {
    let task = task.save(db).await.map_err(|err| {
        eprintln!("Error creating new task: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating task.")
    })?;
    convert_active_to_model(task)
}

fn convert_active_to_model(active_task: tasks::ActiveModel) -> Result<tasks::Model, AppError> {
    active_task.try_into_model().map_err(|err| {
        eprintln!("Error converting task active model to model {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error.")
    })
}

pub async fn get_all_tasks(
    user_id: i32,
    db: &DatabaseConnection,
    get_deleted: bool,
) -> Result<Vec<tasks::Model>, AppError> {
    let mut query = tasks::Entity::find().filter(tasks::Column::UserId.eq(user_id));

    if !get_deleted {
        query = query.filter(tasks::Column::DeletedAt.is_null());
    }

    query.all(db).await.map_err(|err| {
        eprintln!("Error getting all tasks {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error getting all tasks")
    })
}

pub async fn get_default_tasks(db: &DatabaseConnection) -> Result<Vec<tasks::Model>, AppError> {
    tasks::Entity::find()
        .filter(tasks::Column::IsDefault.eq(Some(true)))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(db)
        .await
        .map_err(|err| {
            eprintln!("Error getting default tasks: {err:?}");
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error getting default tasks",
            )
        })
}
