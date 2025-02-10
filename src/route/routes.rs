use crate::web::student_controller;

use axum::{
    routing::{get, post},
    Router,
};


pub fn student_routes() -> Router<()> {
    Router::new()
        .route("/students/page", post(student_controller::list_students_handler))
        .route("/students/detail", 
                 get(student_controller::get_student_handler)
                .post(student_controller::post_student_handler_form))
        .route("/students/create", post(student_controller::create_student_handler))
        .route("/students/update", post(student_controller::update_student_handler))
        .route("/students/delete", get(student_controller::delete_student_handler))
}
