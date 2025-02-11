use std::collections::HashMap;

use crate::api::res::Res;
use crate::ddd::core::{DomainModel, MultipartDomainModel};
use axum::extract::{FromRequest, Multipart, Query, Request};
use axum::{async_trait, Form, Json};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};


#[must_use]
#[derive(Clone, Serialize, Deserialize)]
pub struct ValidJson<T: Clone, DM: Clone + DomainModel<CQES=T>>(pub T, pub DM);

#[async_trait]
impl<T, S, DM> FromRequest<S> for ValidJson<T, DM>
where
    S: Send + Sync,
    T: Serialize + DeserializeOwned + Clone + 'static, // 确保 T 可以解析
    DM: Clone + Send + Sync + DomainModel<CQES=T> + TryFrom<T> +  'static,  // 确保 U 可以进行校验
{
    type Rejection = Res<String>;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {

        let json_result = Json::<T>::from_request(req, state).await.map_err(|_| "解析失败".to_string());

        match json_result {
            Ok(Json(value)) => {
                // 使用 DomainGuard::new(&value) 做校验
                match DM::new(&value) {
                    Ok(dm) => Ok(ValidJson(value, dm)),
                    Err(e) => Err(Res::err(e)),
                }
            }
            Err(e) => Err(Res::err(format!("参数格式非法: {}", e))),
        }
    }
}



#[must_use]
#[derive(Clone, Serialize, Deserialize)]
pub struct ValidQuery<T: Clone, DM: Clone + DomainModel<CQES=T>>(pub T, pub DM);

#[async_trait]
impl<T, S, DM> FromRequest<S> for ValidQuery<T, DM>
where
    S: Send + Sync,
    T: Serialize + DeserializeOwned + Clone + 'static, // 确保 T 可以解析
    DM: Clone + Send + Sync + DomainModel<CQES=T> + TryFrom<T> +  'static,  // 确保 U 可以进行校验
{
    type Rejection = Res<String>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 1. 解析查询参数为 T
        let query_result = Query::<T>::from_request(req, state).await.map_err(|_| "解析失败".to_string());

        match query_result {
            Ok(Query(value)) => {
                // 2. 进行校验并转换为 U
                match DomainModel::new(&value) {
                    Ok(dm) => Ok(ValidQuery(value, dm)),
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
pub struct ValidFile<T, DM: Clone + DomainModel<CQES=T>>(pub T, pub DM);
#[async_trait]
impl<S, T, DM> FromRequest<S> for ValidFile<T, DM>
where
    S: Send + Sync,
    T: Serialize + DeserializeOwned + Clone + 'static, // 确保 T 可以解析
    DM: Clone + Send + Sync + MultipartDomainModel<CQES=T> + TryFrom<T> +  'static,  // 确保 U 可以进行校验
{
    type Rejection = Res<String>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 使用 Multipart 提取表单数据
        let multipart_param = Multipart::from_request(req, state).await.map_err(|_| "解析失败".to_string());
        match multipart_param {
            Ok(mut multipart) => {
                let mut forms = HashMap::new();
                let mut files = Vec::new();

                // 遍历 form 字段
                while let Some(field) = multipart.next_field().await.unwrap() {
                    let name = field.name().unwrap_or("").to_string();
                    let file_name = field.file_name().map(|f| f.to_string()); // 提前获取 filename
                    let field_content_type = field.content_type().map(|ct| ct.to_string()); // 提前获取 Content-Type

                    if file_name.is_none() && field_content_type.is_none() {
                        // 处理文本字段
                        let text = field.text().await.unwrap();
                        forms.insert(name, text);
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
                let forms = serde_json::from_value::<T>(serde_json::to_value(&forms).unwrap());
                match forms {
                    Ok(fields) => {
                        match MultipartDomainModel::new(&fields, files) {
                            Ok(dm) => Ok(ValidFile(fields, dm)),
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
pub struct ValidForm<T, DM: Clone + DomainModel<CQES=T>>(pub T, pub DM);
#[async_trait]
impl<S, T, DM> FromRequest<S> for ValidForm<T, DM>
where
    T: Serialize + DeserializeOwned + Clone,
    DM: DomainModel<CQES=T> + Clone + TryFrom<T> + Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = Res<String>;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 使用 Form 提取表单数据
        let form = Form::<T>::from_request(req, state).await;
        match form {
            Ok(Form(value)) =>  {
                match DomainModel:: new(&value) {
                    Ok(dm) => Ok(ValidForm(value, dm)),
                    Err(e) => Err(Res::err(e)),
                }
            },
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

