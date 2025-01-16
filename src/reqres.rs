use crate::error::Error;
use axum::response::{IntoResponse, Response};
use rbatis::PageRequest;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use axum::extract::FromRef;

const SUCCESS: &'static str  = "10000";

const ERROR: &'static str  = "99999";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageReq<T> {
    pub page_no: Option<u32>,
    pub page_size: Option<u16>,
    pub req: Option<T>,
}

// impl<T> Into<PageRequest> for PageReq<T> {
//     fn into(self) -> PageRequest {
//         PageRequest::new(self.page_no.unwrap_or(1) as u64, self.page_size.unwrap_or(10) as u64)
//     }
// }


impl<T> FromRef<PageReq<T>> for PageRequest {
    fn from_ref(value: &PageReq<T>) -> Self {
        PageRequest::new(value.page_no.unwrap_or(1) as u64, value.page_size.unwrap_or(10) as u64)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageRes<T> {
    pub total: u64,
    pub rows: Option<Vec<T>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Res<T> {
    ok: bool,
    code: &'static str,
    data: Option<T>,
    msg: String,
}

impl<T> Res<T> {
    
    pub fn is_ok(&self) -> bool{
        self.ok
    }

    pub fn is_err(&self) -> bool{
        !self.ok
    }
    
    pub fn ok_of(data: T) -> Self {
        Self {
            ok: true,
            code: SUCCESS,
            data: Some(data),
            msg: "".to_string(),
        }
    }

    

    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            ok: true,
            code: ERROR,
            data: None,
            msg: msg.into()
        }
    }
}


impl Res<()> {
    pub fn ok() -> Self {
        Self {
            ok: true,
            code: SUCCESS,
            data: None,
            msg: "".to_string(),
        }
    }
}

impl<T> From<Result<T, Error>> for Res<T> {
    fn from(value: Result<T, Error>) -> Self {
        match value {
            Ok(data) => Res::ok_of(data),
            Err(e) => Res::err(e.to_string())
        }
    }
}



impl<T> Display for Res<T>
    where T: Serialize + DeserializeOwned + Clone
{
    
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl<T> IntoResponse for Res<T> 
    where T: Serialize + DeserializeOwned 
{
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

