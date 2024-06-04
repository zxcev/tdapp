use dotenvy::dotenv;
use project_solution::{app_state::AppState, run, utils::token_wrapper::TokenWrapper};
use sea_orm::Database;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url: String = dotenvy_macro::dotenv!("DATABASE_URL").to_owned();
    let jwt_secret = dotenvy_macro::dotenv!("JWT_SECRET").to_owned();

    println!("start");
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(err) => {
            eprint!("Error connecting to the database: {err:?}");
            panic!();
        }
    };

    let app_state = AppState {
        db,
        jwt_secret: TokenWrapper(jwt_secret),
    };
    run(app_state).await;
}
