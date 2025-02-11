use serde::{Deserialize, Serialize};

pub struct MultipartFile {
    pub filename: String,
    pub content_type: Option<String>,
    pub data: Vec<u8>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageReq<T: Clone> {
    pub page_no: Option<u32>,
    pub page_size: Option<u16>,
    pub req: Option<T>,
}