
use axum::response::IntoResponse;
use crate::app::dto::IdCommand;
use crate::pool;

use crate::domain::{
    cmd::student_cmd::{StudentCreate, StudentPageQuery, StudentUpdate},
    core::IdOper
};

use crate::app::dto::req::{PageReq, ValidForm, ValidJson, ValidQuery};
use crate::app::dto::res::{JsonRes, PageRes, Res};
use crate::app::dto::student_cmd::{StudentCreateCommand, StudentPageQueryCommand, StudentUpdateCommand};
use crate::domain::core::PageQuery;
use crate::domain::repo::student::StudentRepository;
use crate::infra::po::student::StudentPO;
use crate::infra::repository::student::StudentRepositoryImpl;
use crate::service::student_services::{
    create_student,
    delete_student,
    get_student,
    list_students,
    update_student
};

pub(crate) async fn get_student_handler(
    // State(srb): State<Arc<AppState>>,
    ValidQuery(stu, ..): ValidQuery<IdCommand<i64>, IdOper<i64>>,
) -> impl IntoResponse {
    JsonRes(get_student(pool!(), stu.id.unwrap()).await)
}


pub(crate) async fn post_student_handler_form(
    // State(srb): State<Arc<AppState>>,
    ValidForm(stu, ..): ValidForm<IdCommand<i64>, IdOper<i64>>,
) -> impl IntoResponse {
    JsonRes(get_student(pool!(), stu.id.unwrap()).await)
}


pub(crate) async fn create_student_handler(
    // State(srb): State<Arc<AppState>>,
    ValidJson(_, create): ValidJson<StudentCreateCommand, StudentCreate>,
) -> impl IntoResponse {
    JsonRes(create_student(pool!(), create.into()).await)
}

pub(crate) async fn update_student_handler(
    // State(srb): State<Arc<AppState>>,
    ValidJson(_, update): ValidJson<StudentUpdateCommand, StudentUpdate>,
) -> impl IntoResponse {
    JsonRes(update_student(pool!(), update.into()).await)
}

pub async fn delete_student_handler(
    // State(srb): State<Arc<AppState>>,
    ValidQuery(stu, ..): ValidQuery<IdCommand<i64>, IdOper<i64>>,
) -> impl IntoResponse {
    JsonRes(delete_student(pool!(), stu.id.unwrap()).await)
}

pub async fn list_students_handler(
    // State(srb): State<Arc<AppState>>,
    ValidJson(page_query_command, page): ValidJson<PageReq<StudentPageQueryCommand>, PageQuery<StudentPageQuery>>,
) -> impl IntoResponse {
    let res = StudentRepositoryImpl::find_page(page).await;
    JsonRes(res)
}