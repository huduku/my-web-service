use crate::req::{PageReq, ValidJson};
use crate::res::{JsonOpt, JsonRes, PageRes, Res};
use crate::{models::Student, services::*, AppState};
use axum::response::IntoResponse;
use axum::{
    extract::{Path, State}
    ,
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

pub fn student_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/students", post(list_students_handler))
        .route("/students/:id", get(get_student_handler))
        .route("/students/create", post(create_student_handler))
        .route("/students/:id", put(update_student_handler))
        .route("/students/:id", delete(delete_student_handler))
}

async fn get_student_handler(
    State(srb): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    JsonOpt(get_student(&srb.rbatis, id).await)
}

async fn create_student_handler(
    State(srb): State<Arc<AppState>>,
    ValidJson(student): ValidJson<Student>,
) -> impl IntoResponse {
    JsonRes(create_student(&srb.rbatis, student).await)
}

async fn update_student_handler(
    State(srb): State<Arc<AppState>>,
    Path(id): Path<i64>,
    ValidJson(student): ValidJson<Student>,
) -> impl IntoResponse {
    let mut student = student;
    student.id = Some(id);
    JsonRes(update_student(&srb.rbatis, student).await)
}

pub async fn delete_student_handler(
    State(srb): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    JsonRes(delete_student(&srb.rbatis, id).await)
}

pub async fn list_students_handler(
    State(srb): State<Arc<AppState>>,
    ValidJson(mut req): ValidJson<PageReq<Student>>,
) -> impl IntoResponse {
    req.page_no = Some(req.page_no.unwrap_or(1));
    req.page_size = Some(req.page_size.unwrap_or(10));
    match list_students(&srb.rbatis, req).await {
        Ok(students) => Res::from(PageRes::from(students)),
        Err(e) => Res::err(e.to_string()),
    }
}
