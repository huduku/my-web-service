use serde::{Deserialize, Serialize};
use crate::ddd::core::Safes;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudentCreateCommand {
    pub stu_no: Option<String>,
    pub name: Option<String>,
    pub age: Option<u16>,
    pub class_id: Option<u32>,
    pub address: Option<String>,
}
impl Safes for StudentCreateCommand {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudentUpdateCommand {
    pub id: Option<i64>,
    pub stu_no: Option<String>,
    pub name: Option<String>,
    pub age: Option<u16>,
    pub class_id: Option<u32>,
    pub address: Option<String>,
}

impl Safes for StudentUpdateCommand {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudentPageQueryCommand {
    pub stu_no: Option<String>,
    pub name: Option<String>,
    pub class_id: Option<u32>,
    pub age_start: Option<u16>,
    pub age_end: Option<u16>
}

impl Safes for StudentPageQueryCommand {}

