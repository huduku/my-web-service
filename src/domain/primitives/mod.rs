pub mod students;




use serde::{Deserialize, Serialize};
use crate::domain::core::DomainPrimitive;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Id<T>(pub T);

impl DomainPrimitive<i64> for Id<i64> {
    type Error = String;
    fn new(value: Option<i64>) -> Result<Id<i64>, String> {
        match value {
            Some(v) =>  {
                if v <= 0 {
                    return Err("id 参数非法".to_string());
                }
                Ok(Id(v))
            },
            None => Err("id 不能为空".to_string())
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdOper<T> where T: Send + Sync{
    pub id: Id<T>
}

unsafe impl   Send  for IdOper<i64> {}
unsafe impl   Sync  for IdOper<i64>  {}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageNo(pub u32);

impl DomainPrimitive<u32> for PageNo {
    type Error = String;
    fn new(value: Option<u32>) -> Result<Self, Self::Error>
    where
        Self: Sized
    {
        match value {
            Some(v) => Ok(PageNo(v)),
            None => Ok(PageNo(1)),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageSize(pub u16);


impl DomainPrimitive<u16> for PageSize {

    type Error = String;
    fn new(value: Option<u16>) -> Result<Self, String> {
        match value {
            Some(v) => Ok(PageSize(v)),
            None => Ok(PageSize(10)),
        }
    }
}
