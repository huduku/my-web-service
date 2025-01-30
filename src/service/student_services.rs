use crate::domain::model::student::Student;
use axum::extract::FromRef;

use rbatis::rbatis::RBatis;

use crate::dto::req::PageReq;
use rbatis::rbdc::db::ExecResult;
use rbatis::{Page, PageRequest};
use rbs::Error;

// pub async fn get_student(rb: &RBatis, id: i64) -> Option<Student> {
//     let rows_res = Student::select_by_column(rb, "id", id).await;
//     rows_res.ok().unwrap_or(Vec::new()).pop()
// }

pub async fn get_student(rb: &RBatis, id: i64) -> Result<Student, String> {
    let rows_res = Student::select_by_column(rb, "id", id).await;
    match rows_res {
        Ok(rows) => {
            if rows.len() > 0 {
                return Ok(rows.get(0).unwrap().to_owned());
            }
            return Err("查询数据为空".to_string());
        },
        Err(_) => Err("查询数据库异常".to_string())
    }
}

pub async fn create_student(rb: &RBatis, student: Student) -> Result<ExecResult, Error> {
    Student::insert(rb, &student).await
}

pub async fn update_student(rb: &RBatis, student: Student) -> Result<ExecResult, Error> {
    Student::update_by_column(rb, &student, "id").await
}

pub async fn delete_student(rb: &RBatis, id: i64) -> Result<ExecResult, Error> {
    Student::delete_by_column(rb, "id", id).await
}

pub async fn list_students(
    rb: &RBatis,
    stu_page_req: PageReq<Student>,
) -> rbatis::Result<Page<Student>> {
    let page = PageRequest::from_ref(&stu_page_req);
    let stu = &stu_page_req.req;
    Student::select_page(rb, &page, &stu).await
}
