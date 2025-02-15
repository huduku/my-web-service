use crate::api::cmd::IdCommand;
use crate::app::cmd::student_cmd::{StudentCreate, StudentQuery, StudentUpdate};
use axum::response::IntoResponse;

use crate::infra::web::req::{ValidForm, ValidJson, ValidQuery};

use crate::api::cmd::student_cmd::{StudentCreateCommand, StudentPageQueryCommand, StudentUpdateCommand};
use crate::context::CONTEXT;
use crate::ddd::core::{IdOper, PageQuery};
use crate::ddd::dto::PageReq;
use crate::domain::service::student_services::StudentService;
use crate::infra::repository::student::StudentRepositoryImpl;
use crate::infra::web::res::JsonRes;


fn student_service() -> &'static StudentService<StudentRepositoryImpl> {
    &CONTEXT.app_srv_container.student_service
}

fn student_repository() -> &'static StudentRepositoryImpl {
    CONTEXT.app_repo_context.student_repository
}

pub(crate) async fn get_student_handler(
    ValidQuery(id_query,_): ValidQuery<IdCommand<i64>, IdOper<i64>>,
) -> impl IntoResponse {
    let res = student_service().get_student(id_query).await;
    JsonRes(res)
}


pub(crate) async fn post_student_handler_form(
    ValidForm(id_query, ..): ValidForm<IdCommand<i64>, IdOper<i64>>, ) -> impl IntoResponse {
    JsonRes(student_service().get_student(id_query).await)
}


pub(crate) async fn create_student_handler(
    ValidJson(_, create): ValidJson<StudentCreateCommand, StudentCreate>, ) -> impl IntoResponse {
    student_service().save_student(create.into()).await
}

pub(crate) async fn update_student_handler(
    ValidJson(_, update): ValidJson<StudentUpdateCommand, StudentUpdate>, ) -> impl IntoResponse {
    student_service().save_student(update.into()).await
}

pub async fn delete_student_handler(
    ValidQuery(id_command, ..): ValidQuery<IdCommand<i64>, IdOper<i64>>, ) -> impl IntoResponse {
    student_service().delete_student(id_command).await
}

pub async fn list_students_handler(
    ValidJson(_, page_query): ValidJson<PageReq<StudentPageQueryCommand>, PageQuery<StudentQuery>>, )
    -> impl IntoResponse {
    JsonRes(student_service().list_students(page_query).await)
}