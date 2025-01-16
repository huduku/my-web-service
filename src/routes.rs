use crate::student_controller;
use crate::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

pub fn student_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/students", post(student_controller::list_students_handler))
        .route("/students/:id", get(student_controller::get_student_handler))
        .route("/students/create", post(student_controller::create_student_handler))
        .route("/students/:id", put(student_controller::update_student_handler))
        .route("/students/:id", delete(student_controller::delete_student_handler))
}
