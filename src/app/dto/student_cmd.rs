use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudentCreateCommand {
    pub stu_no: Option<String>,
    pub name: Option<String>,
    pub age: Option<u16>,
    pub class_id: Option<u32>,
    pub address: Option<String>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudentUpdateCommand {
    pub id: Option<i64>,
    pub stu_no: Option<String>,
    pub name: Option<String>,
    pub age: Option<u16>,
    pub class_id: Option<u32>,
    pub address: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudentPageQueryCommand {
    pub stu_no: Option<String>,
    pub name: Option<String>,
    pub class_id: Option<u32>
}

