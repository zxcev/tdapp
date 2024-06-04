use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{database::users, routes::users::ActiveToModel, utils::app_error::AppError};

pub async fn save_active_user(
    user: users::ActiveModel,
    db: &DatabaseConnection,
) -> Result<users::Model, AppError> {
    let user = user
        .save(db)
        .await
        .map_err(|err| {
            let err_msg = err.to_string();
            if err_msg
                .contains("duplicate key value violates unique constraint \"users_username_key\"")
            {
                AppError::new(
                    StatusCode::BAD_REQUEST,
                    "Username already taken, try again with a different user name",
                )
            } else {
                eprintln!("Error creating user: {err:?}");
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, please try again",
                )
            }
        })?
        .convert_active_to_model()?;

    Ok(user)
}

pub async fn find_by_username(
    username: &str,
    db: &DatabaseConnection,
) -> Result<users::Model, AppError> {
    users::Entity::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|err| {
            eprintln!("Error getting user for logging in: {err:?}");
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "incorrect username and/or password",
            )
        })?
        .ok_or_else(|| {
            AppError::new(
                StatusCode::BAD_REQUEST,
                "incorrect username and/or password",
            )
        })
}
