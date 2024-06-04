use std::net::SocketAddr;

use app_state::AppState;
use router::create_router;
use sea_orm::{Database, DatabaseConnection};

pub mod app_state;
mod database;
mod midleware;
mod queries;
mod router;
mod routes;
pub mod utils;

pub async fn run(app_state: AppState) {
    let app = create_router(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
