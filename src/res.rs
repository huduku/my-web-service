use crate::error::Error;
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use rbatis::Page;

const SUCCESS: &'static str = "10000";

const ERROR: &'static str = "99999";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Res<T> {
    ok: bool,
    code: &'static str,
    data: Option<T>,
    msg: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageRes<T> {
    pub total: u64,
    pub rows: Option<Vec<T>>,
}


impl<T> Res<T> {
    pub fn is_ok(&self) -> bool {
        self.ok
    }

    pub fn is_err(&self) -> bool {
        !self.ok
    }

    pub fn ok_with(data: T) -> Self {
        Self {
            ok: true,
            code: SUCCESS,
            data: Some(data),
            msg: "".to_string(),
        }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: ERROR,
            data: None,
            msg: msg.into(),
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
            Ok(data) => Res::ok_with(data),
            Err(e) => Res {
                ok: false,
                code: ERROR,
                data: None,
                msg: e.to_string(),
            },
        }
    }
}

impl<T> From<Option<T>> for Res<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(data) => Res::ok_with(data),
            None => Res::err("结果为空")
        }
    }
}

impl<T> From<Page<T>> for PageRes<T> 
    where T: Send + Sync
{
    fn from(value: Page<T>) -> Self {
        PageRes {
            total: value.total,
            rows: Some(value.records),
        }
    }
}

impl<T> From<PageRes<T>> for Res<PageRes<T>>
{
    fn from(value: PageRes<T>) -> Self {
        Res::ok_with(value)
    }
}

impl<T> Display for Res<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl<T> IntoResponse for Res<T>
where
    T: Serialize + DeserializeOwned,
{
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}


pub struct JsonOpt<T>(pub Option<T>);

impl<T> IntoResponse for JsonOpt<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self.0 {
            Some(data) => axum::Json(Res::ok_with(data)).into_response(),
            None => axum::Json(Res::<()>::err("结果为空")).into_response()
        }
    }
}

pub struct JsonRes<T, E>(pub Result<T, E>);

impl<T, E> IntoResponse for JsonRes<T, E>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self.0 {
            Ok(_) => axum::Json(Res::ok()).into_response(),
            Err(_) => axum::Json(Res::<()>::err("操作失败")).into_response()
        }
    }
}