use std::marker::PhantomData;

use crate::dto::res::Res;
use axum::extract::{FromRef, FromRequest, Query, Request};
use axum::{async_trait, Json};
use rbatis::PageRequest;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use crate::domain::primitives::dp::DomainPrimitive;

#[must_use]
#[derive(Clone, Serialize, Deserialize)]
pub struct ValidJson<T: Clone, DP: Clone>(pub T, pub PhantomData<DP>);

#[must_use]
#[derive(Clone, Serialize, Deserialize)]
pub struct ValidQuery<T: Clone, DP: Clone>(pub T, pub PhantomData<DP>);

#[async_trait]
impl<T, S, DP> FromRequest<S> for ValidJson<T, DP>
where
    T: Serialize + DeserializeOwned + DomainPrimitive<DP> + From<DP> + Clone,
    DP: Clone + TryFrom<T> + Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = Res<String>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {

        let json_result = Json::<T>::from_request(req, state).await;
        
        match json_result {
            Ok(Json(value)) => {
                // 使用 DomainPrimitive::new(&value) 做校验
                match DomainPrimitive::new(&value) {
                    Ok(domain_primitive) => Ok(ValidJson(DP::into(domain_primitive), PhantomData)), 
                    Err(e) => Err(Res::err(e)),
                }
            }
            Err(e) => Err(Res::err(format!("参数格式非法: {}", e.body_text()))),
        }
    }
}


#[async_trait]
impl<T, S, DP> FromRequest<S> for ValidQuery<T, DP>
where
    S: Send + Sync,
    T: Serialize + DeserializeOwned + DomainPrimitive<DP> + From<DP> + Clone, // 确保 T 可以解析
    DP: Clone + TryFrom<T> + Send + Sync + 'static,  // 确保 U 可以进行校验
{
    type Rejection = Res<String>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 1. 解析查询参数为 T
        let query_result = Query::<T>::from_request(req, state).await;

        match query_result {
            Ok(Query(value)) => {
                // 2. 进行校验并转换为 U
                match DomainPrimitive::new(&value) {
                    Ok(domain_primitive) => Ok(ValidQuery(DP::into(domain_primitive), PhantomData)),  
                    Err(e) => Err(Res::err(e)),
                }
            }
            Err(e) => Err(Res::err(format!("参数格式非法: {}", e.body_text()))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageReq<T: Clone> {
    pub page_no: Option<u32>,
    pub page_size: Option<u16>,
    pub req: Option<T>,
}


impl<T: Clone> FromRef<PageReq<T>> for PageRequest {
    fn from_ref(value: &PageReq<T>) -> Self {
        PageRequest::new(
            value.page_no.unwrap_or(1) as u64,
            value.page_size.unwrap_or(10) as u64,
        )
    }
}

