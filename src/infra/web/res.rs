use std::fmt::Display;
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::api::res::Res;

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
            Some(data) => axum::Json(Res::of(data)).into_response(),
            None => axum::Json(Res::<()>::err("数据为空!")).into_response()
        }
    }
}

pub struct JsonRes<T, E>(pub Result<T, E>);

impl<T, E> IntoResponse for JsonRes<T, E>
where
    T: Serialize,
    E: Display
{
    fn into_response(self) -> Response {
        match self.0 {
            Ok(v) => axum::Json(Res::of(v)).into_response(),
            Err(e) => axum::Json(Res::<()>::err(e.to_string())).into_response()
        }
    }
}
