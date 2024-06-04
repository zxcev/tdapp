use axum::{
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    database::users::{self, Entity as Users},
    utils::{app_error::AppError, jwt::validate_token, token_wrapper::TokenWrapper},
};

pub async fn require_authentication<T>(
    State(token_secret): State<TokenWrapper>,
    State(db): State<DatabaseConnection>,
    headers: HeaderMap,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    // 1. extract token from header
    let header_token = if let Some(token) = headers.get("x-auth-token") {
        // err1-1. failed to slicing header(maybe malformed header)
        token.to_str().map_err(|err| {
            eprintln!("Error extracting token from headers: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error reading token")
        })?
    } else {
        // err1-2. token doesn't exist in header
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "not authenticated!",
        ));
    };

    // 2. validate and decode extracted token
    validate_token(&token_secret.0, header_token).map_err(|err| {
        eprintln!("failed to validtate token {err:?}");
        AppError::new(StatusCode::UNAUTHORIZED, "not authenticated!")
    });
    // err2. err handled inside of `validate_token`

    // 3. find user by id from decoded payload
    let user = Users::find()
        .filter(users::Column::Token.eq(Some(header_token.to_owned())))
        .one(&db)
        .await
        // err3-1. failed to access database(network connection or et cetera)
        .map_err(|err| {
            eprintln!("Error getting user by token: {err:?}");
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was a problem getting your account",
            )
        })?;

    // 4-1. if user exists, logged in
    if let Some(user) = user {
        // -> insert `user` into extension
        request.extensions_mut().insert(user);
    } else {
        // err4. if user doesn't exist not logged in
        // return Err(AppError::new(
        //     StatusCode::UNAUTHORIZED,
        //     "You are not authorized for this",
        // ));
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "not authenticated!",
        ));
    }

    Ok(next.run(request).await)
}
