use serde::{Deserialize, Serialize};

use crate::domain::{
    po::student::Student,
    primitives::{
        dp::DomainPrimitive, 
        students::{Address, Age, ClassId, StuNo, UserName}
    }
};




#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudentCreate {
    pub stu_no: StuNo,
    pub name: UserName,
    pub age: Age,
    pub class_id: ClassId,
    pub address: Address,
}

unsafe impl Send for StudentCreate {}
unsafe impl Sync for StudentCreate {}


impl DomainPrimitive<StudentCreate> for Student {
    fn new(value: &Self) -> Result<StudentCreate, String> {
        StudentCreate::try_from(value.clone())
    }
}



impl TryFrom<Student> for StudentCreate {
    type Error = String;
    fn try_from(value: Student) -> Result<Self, Self::Error> {

        let stu_no = StuNo::new(value.stu_no)?;
        let name =  UserName::new(value.name)?;
        let age =  Age::new(value.age)?;
        let class_id =  ClassId::new(value.class_id)?;
        let address = Address::new(value.address)?;

        Ok(Self {
            stu_no,
            name,
            age,
            class_id,
            address,
        })
    }
}

impl From<StudentCreate> for Student {

    fn from(value: StudentCreate) -> Self {
        Self {
            id: None,
            stu_no: Some(value.stu_no.0),
            name: Some(value.name.0),
            age: Some(value.age.0),
            class_id: Some(value.class_id.0),
            address: Some(value.address.0),
            created_at: Some(chrono::NaiveDateTime::default())
        }
    }
}