use crate::res::Res;
use axum::extract::{FromRef, FromRequest, Request};
use axum::{async_trait, Json};
use rbatis::PageRequest;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use crate::dp::DomainPrimitive;

#[must_use]
pub struct ValidJson<T: Clone>(pub T);



// #[async_trait]
// impl<T, S> FromRequest<S> for ValidJson<T>
// where
//     T: Serialize + DeserializeOwned,
//     S: Send + Sync,
// {
//     type Rejection = Res<T>;

//     async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
//         // 使用 `Json` 提取器尝试解析请求体
//         match Json::<T>::from_request(req, state).await {
//             Ok(Json(value)) => Ok(ValidJson(value)),
//             Err(e) => Err(Res::err(String::from("参数格式非法: ") + e.body_text().as_str())),
//         }
//     }
// }

#[async_trait]
impl<T, S> FromRequest<S> for ValidJson<T>
where
    T: Serialize + DeserializeOwned + DomainPrimitive + Clone,
    S: Send + Sync,
{
    type Rejection = Res<String>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 先解析 JSON 成 `TSource`（如 `Student`）
        let json_result = Json::<T>::from_request(req, state).await;
        
        match json_result {
            Ok(Json(value)) => {
                let v = value.clone();
                match DomainPrimitive::new(v.clone()) {
                    Ok(valid_value) => Ok(ValidJson(v.clone())), // 校验通过
                    Err(e) => Err(Res::err(e)),
                }
            }
            Err(e) => Err(Res::err(format!("参数格式非法: {}", e.body_text()))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageReq<T> {
    pub page_no: Option<u32>,
    pub page_size: Option<u16>,
    pub req: Option<T>,
}

impl<T> FromRef<PageReq<T>> for PageRequest {
    fn from_ref(value: &PageReq<T>) -> Self {
        PageRequest::new(
            value.page_no.unwrap_or(1) as u64,
            value.page_size.unwrap_or(10) as u64,
        )
    }
}

