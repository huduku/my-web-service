use crate::api::cmd::IdCommand;
use crate::domain::cmd::student_cmd::{StudentCreate, StudentQuery, StudentUpdate};
use crate::pool;
use axum::response::IntoResponse;

use crate::infra::web::req::{ValidForm, ValidJson, ValidQuery};

use crate::api::cmd::student_cmd::{StudentCreateCommand, StudentPageQueryCommand, StudentUpdateCommand};
use crate::ddd::core::IdOper;
use crate::ddd::core::PageQuery;
use crate::ddd::dto::PageReq;
use crate::domain::service::student_services::StudentService;
use crate::infra::web::res::JsonRes;

pub(crate) async fn get_student_handler(
    ValidQuery(id_query,_): ValidQuery<IdCommand<i64>, IdOper<i64>>,
) -> impl IntoResponse {
    let student_service = StudentService::new();
    let res = student_service.get_student(id_query).await.into()；
    JsonRes(res)
}


pub(crate) async fn post_student_handler_form(
    ValidForm(stu, ..): ValidForm<IdCommand<i64>, IdOper<i64>>, ) -> impl IntoResponse {
    JsonRes(get_student(pool!(), stu.id.unwrap()).await)
}


pub(crate) async fn create_student_handler(
    ValidJson(_, create): ValidJson<StudentCreateCommand, StudentCreate>, ) -> impl IntoResponse {
    JsonRes(create_student(pool!(), create.into()).await)
}

pub(crate) async fn update_student_handler(
    ValidJson(_, update): ValidJson<StudentUpdateCommand, StudentUpdate>, ) -> impl IntoResponse {
    JsonRes(update_student(pool!(), update.into()).await)
}

pub async fn delete_student_handler(
    ValidQuery(stu, ..): ValidQuery<IdCommand<i64>, IdOper<i64>>, ) -> impl IntoResponse {
    JsonRes(delete_student(pool!(), stu.id.unwrap()).await)
}

pub async fn list_students_handler(
    ValidJson(_, page_query): ValidJson<PageReq<StudentPageQueryCommand>, PageQuery<StudentQuery>>, )
    -> impl IntoResponse {
    JsonRes(list_students(page_query).await)
}