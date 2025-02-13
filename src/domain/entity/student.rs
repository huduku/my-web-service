use crate::api::primitive::students::{Address, Age, ClassId, StuNo, UserName};
use crate::ddd::core::{Aggregate, Entity, Id, Identifiable};
use crate::domain::cmd::student_cmd::{StudentCreate, StudentUpdate};
use serde::{Deserialize, Serialize};
use crate::ddd::safe::Safes;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Student {
    pub id: Option<Id<i64>>,
    pub stu_no: Option<StuNo>,
    pub name: UserName,
    pub age: Age,
    pub class_id: ClassId,
    pub address: Address,
    
}



impl Identifiable<Id<i64>> for Student {}
impl Entity<Id<i64>> for Student {}
impl Aggregate<Id<i64>> for Student {}

impl Safes for Student {}

impl From<StudentCreate> for Student {
    fn from(value: StudentCreate) -> Self {
        Self {
            id: None,
            stu_no: Some(value.stu_no),
            name: value.name,
            age: value.age,
            class_id: value.class_id,
            address: value.address
        }
    }
}

impl From<StudentUpdate> for Student {
    fn from(value: StudentUpdate) -> Self {
        Self {
            id: Some(value.id),
            stu_no: None,
            name: value.name,
            age: value.age,
            class_id: value.class_id,
            address: value.address
        }
    }
}