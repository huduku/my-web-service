use std::sync::Arc;
use axum::extract::State;
use axum::response::IntoResponse;
use crate::domain::primitive::dp::IdQuery;
use crate::domain::primitive::students::{StudentCreate, StudentQuery, StudentUpdate};
use crate::AppState;
use crate::domain::model::student::Student;
use crate::dto::req::{PageReq, ValidJson, ValidQuery};
use crate::dto::res::{JsonRes, PageRes, Res};
use crate::service::student_services::{create_student, 
    delete_student, 
    get_student, 
    list_students, 
    update_student 
};

pub(crate) async fn get_student_handler(
    State(srb): State<Arc<AppState>>,
    ValidQuery(stu, ..): ValidQuery<Student, IdQuery>,
) -> impl IntoResponse {
    JsonRes(get_student(&srb.rbatis, stu.id.unwrap()).await)
}

pub(crate) async fn create_student_handler(
    State(srb): State<Arc<AppState>>,
    ValidJson(student, ..): ValidJson<Student, StudentCreate>,
) -> impl IntoResponse {
    JsonRes(create_student(&srb.rbatis, student).await)
}

pub(crate) async fn update_student_handler(
    State(srb): State<Arc<AppState>>,
    ValidJson(student,..): ValidJson<Student, StudentUpdate>,
) -> impl IntoResponse {
    JsonRes(update_student(&srb.rbatis, student).await)
}

pub async fn delete_student_handler(
    State(srb): State<Arc<AppState>>,
    ValidQuery(stu, ..): ValidQuery<Student, IdQuery>,
) -> impl IntoResponse {
    JsonRes(delete_student(&srb.rbatis, stu.id.unwrap()).await)
}

pub async fn list_students_handler(
    State(srb): State<Arc<AppState>>,
    ValidJson(req, ..): ValidJson<PageReq<Student>, StudentQuery>,
) -> impl IntoResponse {
    match list_students(&srb.rbatis, req).await {
        Ok(students) => Res::<PageRes<Student>>::of(students.into()),
        // Ok(students) => Res::of(PageRes::from(students)),
        Err(e) => Res::err(e.to_string()),
    }
}
