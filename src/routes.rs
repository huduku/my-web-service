use crate::reqres::{PageReq, PageRes};
use crate::reqres::Res;
use crate::{models::Student, services::*, AppState};
use axum::response::IntoResponse;
use axum::{extract::{Path, State}, response::Json, routing::{delete, get, post, put}, Router};
use std::sync::Arc;
use axum::extract::rejection::JsonRejection;

pub fn student_routes() ->  Router<Arc<AppState>> {
    Router::new()
        .route("/students", post(list_students_handler))
        .route("/students/:id", get(get_student_handler))
        .route("/students/add", post(create_student_handler))
        .route("/students/:id", put(update_student_handler))
        .route("/students/:id", delete(delete_student_handler))
}

async fn get_student_handler(State(srb): State<Arc<AppState>>, Path(id): Path<i64>) 
    -> impl  IntoResponse {
    match get_student(&srb.rbatis, id).await {
        Some(student) => Res::ok_of(student),
        None => Res::err("Student not found"),
    }
}

async fn create_student_handler(State(srb): State<Arc<AppState>>,Json(student): Json<Student>) 
    -> impl IntoResponse {
    match create_student(&srb.rbatis, student).await {
        Ok(_) => Res::ok(),
        Err(e) =>  Res::err(e.to_string()),
    }
}

async fn update_student_handler(State(srb): State<Arc<AppState>>, Path(id): Path<i64>, Json(student): Json<Student>) 
    -> impl IntoResponse{
    let mut student = student;
    student.id = Some(id);
    match update_student(&srb.rbatis, student).await {
        Ok(_) => Res::ok(),
        Err(e) => Res::err(e.to_string()),
    }
}




pub async fn delete_student_handler(State(srb): State<Arc<AppState>>, Path(id): Path<i64>)
                                   -> impl IntoResponse {
    match delete_student(&srb.rbatis, id).await {
        Ok(_) => Res::ok(),
        Err(e) => Res::err(e.to_string()),
    }
}


pub async fn list_students_handler(State(srb): State<Arc<AppState>>, body: Result<Json<PageReq<Student>>, JsonRejection>)
    -> impl IntoResponse {
    let mut req = match body {
        Ok(Json(stu_req)) => stu_req,
        Err(_) => return Res::err("参数非法")
    };
    
    req.page_no = Some(req.page_no.unwrap_or(1));
    req.page_size = Some(req.page_size.unwrap_or(10));
    match list_students(&srb.rbatis, req).await {
        Ok(students) => {
            let total = students.total;
            let data = students.records;
            let page_res = PageRes {
                total: total,
                rows: Some(data)
            };
            Res::ok_of(page_res)
        },
        Err(e) => Res::err(e.to_string()),
    }
}

#[derive(serde::Deserialize)]
struct PageParams {
    page: Option<u64>,
    page_size: Option<u64>,
}