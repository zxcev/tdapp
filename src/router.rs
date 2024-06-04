use crate::{
    app_state::AppState,
    midleware::require_authentication::require_authentication,
    routes::{
        hello_world::hello_world,
        tasks::{
            create_task::create_task,
            delete_task::{self, soft_delete_task},
            get_all_tasks::get_all_tasks,
            get_one_task::{self, get_one_task},
            update_task::{self, mark_completed, mark_uncompleted, update_task},
        },
        users::{create_user::create_user, login::login, logout::logout},
    },
};
use axum::{
    middleware::{self, from_fn},
    routing::{delete, get, patch, post, put},
    Router,
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        // middleware 위에 있는 route만 적용됨
        .route("/api/v1/tasks/:task_id", get(get_one_task))
        .route("/api/v1/tasks/:task_id/completed", put(mark_completed))
        .route("/api/v1/tasks/:task_id/uncompleted", put(mark_uncompleted))
        .route("/api/v1/tasks/:task_id", patch(update_task))
        .route("/api/v1/tasks/:task_id", delete(soft_delete_task))
        .route("/api/v1/users/logout", post(logout))
        .route("/api/v1/tasks", post(create_task))
        .route("/api/v1/tasks", get(get_all_tasks))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_authentication,
        ))
        .route("/", get(hello_world))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(login))
        // .route_layer(middleware::from_fn_with_state(app_state.clone()))
        .with_state(app_state)
}
