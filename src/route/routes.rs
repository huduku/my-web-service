use crate::controller::student_controller;
use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn student_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/students/page", post(student_controller::list_students_handler))
        .route("/students/detail", get(student_controller::get_student_handler))
        .route("/students/create", post(student_controller::create_student_handler))
        .route("/students/update", post(student_controller::update_student_handler))
        .route("/students/delete", get(student_controller::delete_student_handler))
}
