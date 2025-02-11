
use crate::pool;
use axum::response::IntoResponse;
use crate::api::cmd::IdCommand;
use crate::domain::cmd::student_cmd::{StudentCreate, StudentPageQuery, StudentUpdate};

use crate::infra::web::req::{ValidForm, ValidJson, ValidQuery};

use crate::api::cmd::student_cmd::{StudentCreateCommand, StudentPageQueryCommand, StudentUpdateCommand};
use crate::ddd::core::IdOper;
use crate::ddd::core::PageQuery;
use crate::ddd::dto::PageReq;
use crate::domain::repo::student::StudentRepository;
use crate::infra::repository::student::StudentRepositoryImpl;
use crate::infra::web::res::JsonRes;
use crate::service::student_services::{
    create_student,
    delete_student,
    get_student
    ,
    update_student
};

pub(crate) async fn get_student_handler(
    ValidQuery(stu, ..): ValidQuery<IdCommand<i64>, IdOper<i64>>,
) -> impl IntoResponse {
    JsonRes(get_student(pool!(), stu.id.unwrap()).await)
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
    ValidJson(_, page): ValidJson<PageReq<StudentPageQueryCommand>, PageQuery<StudentPageQuery>>, ) 
    -> impl IntoResponse {
    let res = StudentRepositoryImpl::find_page(page).await;
    JsonRes(res)
}