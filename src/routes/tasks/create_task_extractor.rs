use axum::{
    async_trait,
    body::{Bytes, HttpBody},
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError, Json, RequestExt,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::app_error::AppError;

#[derive(Debug, Validate, Deserialize)]
pub struct ValidateCreateTask {
    pub description: Option<String>,
    #[validate(length(min = 1, max = 1, message = "Priority must be a single character"))]
    pub priority: Option<String>,
    #[validate(required(message = "missing task title"))]
    pub title: Option<String>,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for ValidateCreateTask
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
    // Bytes: FromRequest<S, B>,
    // B: Send + 'static,
    // S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(
        req: Request<B>,
        state: &S,
    ) -> Result<ValidateCreateTask, Self::Rejection> {
        // let body = Bytes::from_request(req, state).await.map_err(|_err| {
        //     eprintln!("Error getting bytes in custom create task extractor");
        //     AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "")
        // })?;
        let Json(task) = req
            .extract::<Json<ValidateCreateTask>, _>()
            .await
            .map_err(|err| {
                eprintln!("Error extracting new task {err:?}");
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, please try again",
                )
            })?;
        // let body = Json::<ValidateCreateTask>::from_request(req, state)
        //     .await
        //     .map_err(|_err| {
        //         eprintln!("Error getting bytes in custom create task extractor");
        //         AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "")
        //     })?;

        if let Err(errs) = task.validate() {
            // dbg!(errs.to_string());
            let field_errors = errs.field_errors();
            for (_, error) in field_errors {
                // feel safe unwrapping because we know there is at least one error,
                // and we only care about the first for this api
                let error_message = error.first().unwrap().message.clone().unwrap().to_string();
                return Err(AppError::new(StatusCode::BAD_REQUEST, error_message));
            }
        };

        Ok(task)
    }
}
