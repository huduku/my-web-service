use serde::{Deserialize, Serialize};
use crate::domain::core::{Aggregate, DomainModel, DomainPrimitive, Entity, Id, Identifiable};
use crate::domain::primitive::students::{Address, Age, ClassId, StuNo, UserName};
use crate::infra::po::student::StudentPO;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Student {

    pub id: Id<i64>,
    pub stu_no: StuNo,
    pub name: UserName,
    pub age: Age,
    pub class_id: ClassId,
    pub address: Address,
    
}

impl DomainModel for Student {
    type CQES = StudentPO;
    fn new(value: &Self::CQES) -> Result<Self, String> {
        let value = value.to_owned();
        Ok(Student {
            id: Id::new(value.id)?,
            stu_no: StuNo::new(value.stu_no)?,
            name: UserName::new(value.name)?,
            age: Age::new(value.age)?,
            class_id: ClassId::new(value.class_id)?,
            address: Address::new(value.address)?,
        })
    }
}

impl Identifiable<Id<i64>> for Student {}
impl Entity<Id<i64>> for Student {}
impl Aggregate<Id<i64>> for Student {}