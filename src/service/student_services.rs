
use crate::infra::po::student::StudentPO;
use crate::app::dto::res::DbRes;
use axum::extract::FromRef;

use rbatis::{rbatis::RBatis, rbdc::db::ExecResult};

use crate::app::dto::req::PageReq;
use rbatis::{Page, PageRequest};



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

pub async fn list_students(rb: &RBatis, stu_page_req: PageReq<StudentPO>,)
                           -> Result<Page<StudentPO>, String> {
    let page = PageRequest::from_ref(&stu_page_req);
    let stu = &stu_page_req.req;
    let DbRes(res) = StudentPO::select_page(rb, &page, stu).await.into();
    res
}