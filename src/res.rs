use crate::error::Error;
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

const SUCCESS: &'static str = "10000";

const ERROR: &'static str = "99999";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Res<T> {
    ok: bool,
    code: &'static str,
    data: Option<T>,
    msg: String,
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
