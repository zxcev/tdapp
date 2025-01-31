use axum::http::StatusCode;
use sea_orm::TryIntoModel;
use serde::{Deserialize, Serialize};

use crate::{
    database::{tasks::ActiveModel, users},
    utils::app_error::AppError,
};

pub mod create_user;
pub mod login;
pub mod logout;

#[derive(Serialize, Deserialize)]
pub struct ResponseDataUser {
    data: ResponseUser,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    id: i32,
    username: String,
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCreateUser {
    username: String,
    password: String,
}

fn convert_active_to_model(user: users::ActiveModel) -> Result<users::Model, AppError> {
    user.try_into_model().map_err(|err| {
        eprintln!("Error converting user back into model: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating user")
    })
}

pub trait ActiveToModel {
    fn convert_active_to_model(self) -> Result<users::Model, AppError>;
}

impl ActiveToModel for users::ActiveModel {
    fn convert_active_to_model(self) -> Result<users::Model, AppError> {
        self.try_into_model().map_err(|err| {
            eprintln!("Error converting user back into model: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating user")
        })
    }
}
