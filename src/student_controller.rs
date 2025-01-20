use std::sync::Arc;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use crate::AppState;
use crate::models::Student;
use crate::req::{PageReq, ValidJson};
use crate::res::{JsonOpt, JsonRes, PageRes, Res};
use crate::services::{create_student, delete_student, get_student, list_students, update_student};

pub(crate) async fn get_student_handler(
    State(srb): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    JsonOpt(get_student(&srb.rbatis, id).await)
}

pub(crate) async fn create_student_handler(
    State(srb): State<Arc<AppState>>,
    ValidJson(student): ValidJson<Student>,
) -> impl IntoResponse {
    JsonRes(create_student(&srb.rbatis, student).await)
}

pub(crate) async fn update_student_handler(
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
        Ok(students) => Res::of(PageRes::from(students)),
        Err(e) => Res::err(e.to_string()),
    }
}
