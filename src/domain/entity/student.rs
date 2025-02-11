use crate::api::primitive::students::{Address, Age, ClassId, StuNo, UserName};
use crate::ddd::core::{Aggregate, DomainPrimitive, Entity, Id, Identifiable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Student {
    pub id: Id<i64>,
    pub stu_no: StuNo,
    pub name: UserName,
    pub age: Age,
    pub class_id: ClassId,
    pub address: Address,
    
}


impl Identifiable<Id<i64>> for Student {}
impl Entity<Id<i64>> for Student {}
impl Aggregate<Id<i64>> for Student {}