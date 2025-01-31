
use serde::{Deserialize, Serialize};

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


impl StuNo {
    pub fn new(value: Option<String>) -> Result<Self, String> {
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

impl StuNoQuery {
    pub fn new(value: Option<String>) -> Result<Self, String> {
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

impl UserName {
    pub fn new(value: Option<String>) -> Result<Self, String> {
        match value {
            Some(v) => Ok(UserName(v)),
            None => Err("name 不能为空".to_string())
        }
    }
}

impl UserNameQuery {
    pub fn new(value: Option<String>) -> Result<Self, String> {
        match value {
            Some(v) => {
                if v.is_empty() {
                    return Ok(UserNameQuery(None));
                }
                if v.chars().count() < 3 {
                    return Err("name必须为空或者长度不能小于3".to_string());
                }
                Ok(UserNameQuery(Some(v)))
            },
            None => Ok(UserNameQuery(None))
        }
    }
}


impl Age {
    pub fn new(value: Option<u16>) -> Result<Self, String> {
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

impl ClassId {
    pub fn new(value: Option<u32>) -> Result<Self, String> {
        match value {
            Some(v) => Ok(ClassId(v)),
            None => Err("class_id 不能为空".to_string())
        }
    }
}


impl ClassIdQuery {
    pub fn new(value: Option<u32>) -> Result<Self, String> {
        match value {
            Some(v) => Ok(ClassIdQuery(Some(v))),
            None => Ok(ClassIdQuery(None))
        }
    }
}

impl Address {
    pub fn new(value: Option<String>) -> Result<Self, String> {
        match value {
            Some(v) => Ok(Address(v)),
            None => Err("address 不能为空".to_string())
        }
    }
}



// #[test]
// fn test() {
//     let stu = Student {
//         id: Some(0),
//         stu_no: Some("10247015".to_string()),
//         name: Some("张三".to_string()),
//         age: Some(180),
//         class_id: Some(5),
//         address:  Some("学号".to_string())
//     };

//     let stu = StudentCreate::try_from(stu);
//     if stu.is_ok() {
//         println!("ok: {}", serde_json::to_string(&stu.unwrap()).unwrap());
//     } else {
//         println!("err: {}", &stu.err().unwrap());
//     }

// }