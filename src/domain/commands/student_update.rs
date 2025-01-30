use serde::{Deserialize, Serialize};

use crate::domain::{
    models::student::Student, 
    primitives::{
        dp::{DomainPrimitive, Id}, 
        students::{Address, Age, ClassId, UserName}
    }
};





#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudentUpdate {
    pub id: Id,
    // pub stu_no: StuNo,
    pub name: UserName,
    pub age: Age,
    pub class_id: ClassId,
    pub address: Address,
}

unsafe impl Send for StudentUpdate {}
unsafe impl Sync for StudentUpdate {}


impl DomainPrimitive<StudentUpdate> for Student {
    fn new(value: &Self) -> Result<StudentUpdate, String> {
        StudentUpdate::try_from(value.clone())
    }
}




impl TryFrom<Student> for StudentUpdate {
    type Error = String;
    fn try_from(value: Student) -> Result<Self, Self::Error> {

        let id = Id::new(value.id)?;
        let name =  UserName::new(value.name)?;
        let age =  Age::new(value.age)?;
        let class_id =  ClassId::new(value.class_id)?;
        let address = Address::new(value.address)?;

        Ok(Self {
            id,
            name,
            age,
            class_id,
            address,
        })
    }
}


impl From<StudentUpdate> for Student {

    fn from(value: StudentUpdate) -> Self {
        Self {
            id: Some(value.id.0),
            stu_no: None,// 唯一索引， 不能更新此字段
            name: Some(value.name.0),
            age: Some(value.age.0),
            class_id: Some(value.class_id.0),
            address: Some(value.address.0)
        }
    }
}
