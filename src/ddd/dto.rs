use serde::{Deserialize, Serialize};

pub struct MultipartFile {
    pub filename: String,
    pub content_type: Option<String>,
    pub data: Vec<u8>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageReq<T> {
    pub page_no: Option<i64>,
    pub page_size: Option<i32>,
    pub req: Option<T>,
}