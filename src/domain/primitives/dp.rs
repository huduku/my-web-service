use crate::dto::req::MultipartFile;
use serde::{Deserialize, Serialize};

pub trait DomainPrimitive<DP> 
    where DP: Clone + Send + Sync 
{
    fn new(value: &Self) -> Result<DP, String>;
}

pub trait MultipartDomainPrimitive<DP>: DomainPrimitive<DP>
    where DP: Clone + Send + Sync
{
    fn new(value: &Self,  multipart_files: Vec<MultipartFile>) -> Result<DP, String>;
}

// impl<DP: Clone> DomainPrimitive<DP> for DP {
//     fn new(value: &DP) -> Result<Self, String>
//     where
//         Self: Sized,
//     {
//         Ok(value.clone())
//     }
// }


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdOper {
    pub id: Id
}

unsafe impl Send for IdOper {}
unsafe impl Sync for IdOper {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Id(pub i64);


impl Id {
    pub fn new(value: Option<i64>) -> Result<Self, String> {
        match value {
            Some(v) =>  {
                if v <= 0 {
                    return Err("id 参数非法".to_string());
                }
                return Ok(Id(v));
            },
            None => Err("id 不能为空".to_string())
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageNo(pub u32);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageSize(pub u16);

impl PageNo {
    pub fn new(value: Option<u32>) -> Result<Self, String> {
        match value {
            Some(v) => Ok(PageNo(v)),
            None => Ok(PageNo(1)),
        }
    }
}

impl PageSize {
    pub fn new(value: Option<u16>) -> Result<Self, String> {
        match value {
            Some(v) => Ok(PageSize(v)),
            None => Ok(PageSize(10)),
        }
    }
}
