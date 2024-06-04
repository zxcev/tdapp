use axum::{extract::State, http::StatusCode, Extension};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, IntoActiveModel, Set};

use crate::{
    database::users,
    queries::user_queries,
    utils::{app_error::AppError, token_wrapper::TokenWrapper},
};

pub async fn logout(
    Extension(user): Extension<users::Model>,
    State(db): State<DatabaseConnection>,
) -> Result<StatusCode, AppError> {
    let mut user = user.into_active_model();

    user.token = Set(None);

    user_queries::save_active_user(user, &db).await?;

    Ok(StatusCode::OK)
}
