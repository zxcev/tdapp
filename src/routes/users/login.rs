use axum::http::StatusCode;
use axum::{extract::State, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::database::users::{self, Entity as Users};
use crate::queries::user_queries;
use crate::routes::users::ResponseUser;
use crate::utils::app_error::AppError;
use crate::utils::hash::verify_password;
use crate::utils::jwt::create_token;
use crate::utils::token_wrapper::TokenWrapper;

use super::{ActiveToModel, RequestCreateUser, ResponseDataUser};

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(jwt_secret): State<TokenWrapper>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let user = user_queries::find_by_username(&request_user.username, &db).await?;

    if !verify_password(&request_user.password, &user.password)? {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "incorrect username and/or password",
        ));
    }

    let token = create_token(&jwt_secret.0, user.username.clone())?;
    let mut user = user.into_active_model();
    user.token = Set(Some(token));
    let user = user_queries::save_active_user(user, &db).await?;

    let response = ResponseUser {
        id: user.id,
        username: user.username,
        token: user.token.unwrap(),
    };

    Ok(Json(ResponseDataUser { data: response }))
}
