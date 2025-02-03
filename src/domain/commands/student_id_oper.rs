use crate::domain::{
    po::student::Student,
    primitives::dp::{DomainPrimitive, Id, IdOper}
};


impl DomainPrimitive<IdOper> for Student {
    fn new(value: &Self) -> Result<IdOper, String> {
        IdOper::try_from(value.clone())
    }
}


impl TryFrom<Student> for IdOper {
    type Error = String;
    fn try_from(value: Student) -> Result<Self, Self::Error> {
        let id = Id::new(value.id)?;
        Ok(IdOper {
            id 
        })
    }
}

impl From<IdOper> for Student {

    fn from(value: IdOper) -> Self {
        Self {
            id: Some(value.id.0),
            stu_no: None,
            name: None,
            age: None,
            class_id: None,
            address: None
        }
    }
}
