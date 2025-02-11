use crate::infra::po::student::StudentPO;

use rbatis::{rbatis::RBatis, rbdc::db::ExecResult};

use crate::api::res::PageRes;
use crate::ddd::core::PageQuery;
use crate::domain::cmd::student_cmd::StudentPageQuery;
use crate::domain::entity::student::Student;
use crate::domain::repo::student::StudentRepository;
use crate::infra::repository::student::StudentRepositoryImpl;
use crate::infra::repository::DbRes;

pub async fn get_student(rb: &RBatis, id: i64) -> Result<StudentPO, String> {
    let DbRes(res) = StudentPO::select_by_id(rb, id).await.into();
    // let res = res?;
    match res? {
        Some(row) => Ok(row),
        None => Err("数据为空!".to_string())
    }
}

pub async fn create_student(rb: &RBatis, student: StudentPO) -> Result<ExecResult, String> {
    let DbRes(res) = StudentPO::insert(rb, &student).await.into();
    res
}

pub async fn update_student(rb: &RBatis, student: StudentPO) -> Result<ExecResult, String> {
    let DbRes(res) = StudentPO::update_by_column(rb, &student, "id").await.into();
    res
}

pub async fn delete_student(rb: &RBatis, id: i64) -> Result<ExecResult, String> {
    let DbRes(res) = StudentPO::delete_by_column(rb, "id", id).await.into();
    res
}

pub async fn list_students(page_query: PageQuery<StudentPageQuery>,)
                           -> Result<PageRes<Student>, String> {
    StudentRepositoryImpl::find_page(page_query).await
}