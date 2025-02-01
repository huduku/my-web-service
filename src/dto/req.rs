use std::collections::HashMap;
use std::marker::PhantomData;

use crate::dto::res::Res;
use axum::extract::{FromRef, FromRequest, Multipart, Query, Request};
use axum::{async_trait, Form, Json};
use rbatis::PageRequest;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use crate::domain::primitives::dp::{DomainPrimitive, MultipartDomainPrimitive};

#[must_use]
#[derive(Clone, Serialize, Deserialize)]
pub struct ValidJson<T: Clone, DP: Clone>(pub T, pub PhantomData<DP>);


#[async_trait]
impl<T, S, DP> FromRequest<S> for ValidJson<T, DP>
where
    T: Serialize + DeserializeOwned + DomainPrimitive<DP> + From<DP> + Clone + Send + Sync + 'static,
    DP: Clone + TryFrom<T> + Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = Res<String>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {

        let json_result = Json::<T>::from_request(req, state).await.map_err(|_| "解析失败".to_string());

        match json_result {
            Ok(Json(value)) => {
                // 使用 DomainPrimitive::new(&value) 做校验
                match DomainPrimitive::new(&value) {
                    Ok(domain_primitive) => Ok(ValidJson(DP::into(domain_primitive), PhantomData)),
                    Err(e) => Err(Res::err(e)),
                }
            }
            Err(e) => Err(Res::err(format!("参数格式非法: {}", e))),
        }
    }
}


#[must_use]
#[derive(Clone, Serialize, Deserialize)]
pub struct ValidQuery<T: Clone, DP: Clone>(pub T, pub PhantomData<DP>);

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
        let query_result = Query::<T>::from_request(req, state).await.map_err(|_| "解析失败".to_string());

        match query_result {
            Ok(Query(value)) => {
                // 2. 进行校验并转换为 U
                match DomainPrimitive::new(&value) {
                    Ok(domain_primitive) => Ok(ValidQuery(DP::into(domain_primitive), PhantomData)),
                    Err(e) => Err(Res::err(e)),
                }
            }
            Err(e) => Err(Res::err(format!("参数格式非法: {}", e))),
        }
    }
}

pub struct MultipartFile {
    pub filename: String,
    pub content_type: Option<String>,
    pub data: Vec<u8>,
}




#[derive(Clone, Serialize, Deserialize)]
pub struct ValidFile<T, DP: Clone>(pub T, pub PhantomData<DP>);
#[async_trait]
impl<S, T, DP> FromRequest<S> for ValidFile<T, DP>
where
    T: Serialize + DeserializeOwned + MultipartDomainPrimitive<DP> + From<DP> + Clone,
    DP: Serialize + DeserializeOwned + Clone + TryFrom<T> + Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = Res<String>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 使用 Multipart 提取表单数据
        let multipart_param = Multipart::from_request(req, state).await.map_err(|_| "解析失败".to_string());
        match multipart_param {
            Ok(mut multipart) => {
                let mut form_data = HashMap::new();
                let mut files = Vec::new();

                // 遍历 form 字段
                while let Some(field) = multipart.next_field().await.unwrap() {
                    let name = field.name().unwrap_or("").to_string();
                    let file_name = field.file_name().map(|f| f.to_string()); // 提前获取 filename
                    let field_content_type = field.content_type().map(|ct| ct.to_string()); // 提前获取 Content-Type

                    if file_name.is_none() && field_content_type.is_none() {
                        // 处理文本字段
                        let text = field.text().await.unwrap();
                        form_data.insert(name, text);
                    } else {
                        // 处理文件
                        let bytes = field.bytes().await.unwrap().to_vec(); // 读取数据
                        files.push(MultipartFile {
                            filename: file_name.unwrap_or_else(|| "".to_string()), // 使用提前获取的 filename
                            content_type: field_content_type, // 使用提前获取的 content_type
                            data: bytes,
                        });
                    }
                }

                // 解析表单字段
                let form_data = serde_json::from_value::<T>(serde_json::to_value(&form_data).unwrap());
                match form_data {
                    Ok(t) => {
                        match MultipartDomainPrimitive::new(&t, files) {
                            Ok(domain_primitive) => Ok(ValidFile(DP::into(domain_primitive), PhantomData)),
                            Err(e) => Err(Res::err(e))
                        }
                    },
                    Err(e) => Err(Res::err(e.to_string()))
                }


            }
            Err(e) => Err(Res::err(format!("参数格式非法: {}", e))),
        }


    }
}


#[derive(Clone, Serialize, Deserialize)]
pub struct ValidForm<T, DP: Clone>(pub T, pub PhantomData<DP>);
#[async_trait]
impl<S, T, DP> FromRequest<S> for ValidForm<T, DP>
where
    T: Serialize + DeserializeOwned + DomainPrimitive<DP> + From<DP> + Clone,
    DP: Clone + TryFrom<T> + Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = Res<String>;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 使用 Form 提取表单数据
        let form = Form::<T>::from_request(req, state).await;
        match form {
            Ok(Form(value)) =>  {
                match DomainPrimitive::new(&value) {
                    Ok(domain_primitive) => Ok(ValidForm(DP::into(domain_primitive), PhantomData)),
                    Err(e) => Err(Res::err(e)),
                }
            },
            Err(e) => Err(Res::err(format!("参数格式非法: {}", e.body_text()))),
        }
    }
}



#[async_trait]
pub trait UnifiedExtractor<S>: Sized
where
    S: Send + Sync,
{
    async fn extract(req: Request, state: &S) -> Result<Self, Res<String>>;
}

#[async_trait]
impl<S, T> UnifiedExtractor<S> for T
where
    S: Send + Sync,
    T: Serialize + DeserializeOwned + Clone,
{
    async fn extract(req: Request, state: &S) -> Result<Self, Res<String>> {
        let content_type = req.headers().get("content-type").and_then(|v| v.to_str().ok());

        if let Some(content_type) = content_type {
            let content_type = content_type.to_lowercase();
            if content_type.contains("application/json") {
                return match Json::<T>::from_request(req, state).await {
                    Ok(Json(value)) => Ok(value),
                    Err(e) => Err(Res::err(format!("JSON 参数格式非法: {}", e.body_text()))),
                };
            } else if content_type.to_lowercase().contains("application/x-www-form-urlencoded") {
                return match Form::<T>::from_request(req, state).await {
                    Ok(Form(value)) => Ok(value),
                    Err(e) => Err(Res::err(format!("Form 参数格式非法: {}", e.body_text()))),
                };
            }
        }

        match Query::<T>::from_request(req, state).await {
            Ok(Query(value)) => Ok(value),
            Err(e) => Err(Res::err(format!("Query 参数格式非法: {}", e.body_text()))),
        }
    }
}


#[must_use]
#[derive(Clone, Serialize, Deserialize)]
pub struct Valid<T: Clone, DP: Clone>(pub T, pub PhantomData<DP>);

#[async_trait]
impl<T, S, DP> FromRequest<S> for Valid<T, DP>
where
    S: Send + Sync,
    T: Serialize + DeserializeOwned + DomainPrimitive<DP> + From<DP> + Clone,
    DP: Clone + TryFrom<T> + Send + Sync + 'static,
{
    type Rejection = Res<String>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let value: T = UnifiedExtractor::extract(req, state).await?;
        match DomainPrimitive::new(&value) {
            Ok(domain_primitive) => Ok(Valid(DP::into(domain_primitive), PhantomData)),
            Err(e) => Err(Res::err(e)),
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

