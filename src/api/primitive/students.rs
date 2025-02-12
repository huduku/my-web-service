
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;
use crate::ddd::core::DomainPrimitive;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StuNo(pub String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StuNoQuery(pub Option<String>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserName(pub String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserNameQuery(pub Option<String>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Age(pub u16);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClassId(pub u32);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClassIdQuery(pub Option<u32>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address(pub String);


impl DomainPrimitive<String> for StuNo {

    type Error = String;
    fn new(value: Option<String>) -> Result<Self, String> {
        match value {
            Some(v) => {
                if v.is_empty() {
                    return Err("stu_no 不能为空值".to_string());
                }
                if v.chars().count() < 8 {
                    return Err("stu_no 长度不能小于 8".to_string());
                }
                if !v.chars().all(|c| c.is_ascii_alphanumeric()) {
                    return Err("stu_no 只能由字母和数字组成".to_string());
                }
                Ok(StuNo(v))
            },
            None => Err("stu_no 不能为空".to_string())
        }
    }
}

impl DomainPrimitive<String> for StuNoQuery {
    type Error = String;
    fn new(value: Option<String>) -> Result<Self, String> {
        match value {
            Some(v) => {
                if v.is_empty() {
                    return Ok(StuNoQuery(None));
                }
                Ok(StuNoQuery(Some(v)))
            },
            None => Ok(StuNoQuery(None))
        }
    }
}


impl DomainPrimitive<String> for UserName {
    type Error = String;

    fn new(value: Option<String>) -> Result<Self, Self::Error>
    where
        Self: Sized
    {
        match value {
            Some(v) => Ok(UserName(v)),
            None => Err("name 不能为空".to_string()) // Ok(UserName("".to_string()))
        }
    }
}

impl DomainPrimitive<String> for UserNameQuery {
    type Error = String;
    fn new(value: Option<String>) -> Result<Self, String> {
        match value {
            Some(v) => {
                if v.is_empty() {
                    return Ok(UserNameQuery(None));
                }
                if v.graphemes(true).count() < 2 {
                    return Err("name必须为空或者长度不能小于2".to_string());
                }
                Ok(UserNameQuery(Some(v)))
            },
            None => Ok(UserNameQuery(None))
        }
    }
}


impl DomainPrimitive<u16> for Age {
    type Error = String;
    fn new(value: Option<u16>) -> Result<Self, String> {
        match value {
            Some(v) => {
                if v > 180 {
                    return Err("age 不能大于 127".to_string());
                }
                Ok(Age(v))
            },
            None => Err("age 不能为空".to_string())
        }
    }
}

impl DomainPrimitive<u32> for ClassId {
    type Error = String;
    fn new(value: Option<u32>) -> Result<Self, String> {
        match value {
            Some(v) => Ok(ClassId(v)),
            None => Err("class_id 不能为空".to_string())
        }
    }
}


impl DomainPrimitive<u32> for ClassIdQuery {

    type Error = String;
    
    fn new(value: Option<u32>) -> Result<Self, String> {
        match value {
            Some(v) => Ok(ClassIdQuery(Some(v))),
            None => Ok(ClassIdQuery(None))
        }
    }
}

impl DomainPrimitive<String> for Address {

    type Error = String;
    fn new(value: Option<String>) -> Result<Self, String> {
        match value {
            Some(v) => Ok(Address(v)),
            None => Err("address 不能为空".to_string())
        }
    }
}

