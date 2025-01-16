use axum::extract::FromRef;
use crate::models::Student;
use crate::reqres::*;

use rbatis::rbatis::RBatis;

use rbatis::rbdc::db::ExecResult;
use rbatis::{Page, PageRequest};
use rbs::Error;

pub async fn get_student(rb: &RBatis, id: i64) -> Option<Student> {
    let rows = Student::select_by_column(rb,"id",id).await;
    rows.ok().unwrap().pop()
}

pub async fn create_student(rb: &RBatis, student: Student) ->  Result<ExecResult, Error> {
    Student::insert(rb, &student).await
}

pub async fn update_student(rb: &RBatis, student: Student) ->  Result<ExecResult, Error> {
    Student::update_by_column(rb, &student, "id").await
}

pub async fn delete_student(rb: &RBatis, id: i64) ->  Result<ExecResult, Error>{
    Student::delete_by_column(rb,"id", id).await
}


pub async fn list_students(
    rb: &RBatis,
    stu_page_req: PageReq<Student>
) -> rbatis::Result<Page<Student>> {
    let page = PageRequest::from_ref(&stu_page_req);
    let stu = &stu_page_req.req;
    Student::select_page(rb, &page,  &stu).await
}