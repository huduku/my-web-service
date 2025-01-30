use std::marker::PhantomData;

use crate::res::Res;
use axum::extract::{FromRef, FromRequest, Request};
use axum::{async_trait, Json};
use rbatis::PageRequest;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use crate::dp::DomainPrimitive;

#[must_use]
pub struct ValidJson<T: Clone, DP: Clone>(pub T, pub PhantomData<DP>);


#[async_trait]
impl<T, S, DP> FromRequest<S> for ValidJson<T, DP>
where
    T: Serialize + DeserializeOwned + DomainPrimitive<DP> + From<DP> + Clone,
    DP: Clone + TryFrom<T> + Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = Res<String>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 先解析 JSON 成 `TSource`（如 `Student`）
        let json_result = Json::<T>::from_request(req, state).await;
        
        match json_result {
            Ok(Json(value)) => {
                match DomainPrimitive::new(&value) {
                    Ok(_) => Ok(ValidJson(value, PhantomData)), 
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

