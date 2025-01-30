use crate::domain::{
    models::student::Student, 
    primitives::dp::{DomainPrimitive, Id, IdQuery}
};


impl DomainPrimitive<IdQuery> for Student {
    fn new(value: &Self) -> Result<IdQuery, String> {
        IdQuery::try_from(value.clone())
    }
}


impl TryFrom<Student> for IdQuery {
    type Error = String;
    fn try_from(value: Student) -> Result<Self, Self::Error> {
        let id = Id::new(value.id)?;
        Ok(IdQuery {
            id 
        })
    }
}

impl From<IdQuery> for Student {

    fn from(value: IdQuery) -> Self {
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
