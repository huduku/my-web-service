use crate::domain::core::{Aggregate, DomainModel, Entity, Id, Identifiable};
use crate::domain::primitive::students::{Address, Age, ClassId, UserName};
use crate::infra::po::student::StudentPO;

pub struct Student {

    pub id: Id<i64>,
    // pub stu_no: StuNo,
    pub name: UserName,
    pub age: Age,
    pub class_id: ClassId,
    pub address: Address,
    
}

impl DomainModel for Student {
    type CQES = StudentPO;
    fn new(value: &Self::CQES) -> Result<Self, String> {
        todo!()
    }
}

impl Identifiable<Id<i64>> for Student {}
impl Entity<Id<i64>> for Student {}
impl Aggregate<Id<i64>> for Student {}