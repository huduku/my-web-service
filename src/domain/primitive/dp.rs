
use crate::domain::model::student::Student;
use crate::domain::primitive::students::{StudentCreate, StudentUpdate};

pub trait DomainPrimitive<DP> {
    fn new(value: &Self) -> Result<DP, String>;
}


impl<DP: Clone> DomainPrimitive<DP> for DP {

    fn new(value : &DP) -> Result<Self, String> where Self: Sized {
        Ok(value.clone())
    }
}

impl DomainPrimitive<StudentCreate> for Student {
    fn new(value: &Self) -> Result<StudentCreate, String> {
        StudentCreate::try_from(value.clone())
    }
}

impl DomainPrimitive<StudentUpdate> for Student {
    fn new(value: &Self) -> Result<StudentUpdate, String> {
        StudentUpdate::try_from(value.clone())
    }
}