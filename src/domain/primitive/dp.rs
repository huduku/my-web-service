
use serde::{Deserialize, Serialize};

use crate::domain::model::student::Student;
use crate::domain::primitive::students::{StudentCreate, StudentUpdate};
use crate::dto::req::PageReq;

use super::students::StudentQuery;

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

impl DomainPrimitive<StudentQuery> for PageReq<Student> {
    fn new(value: &Self) -> Result<StudentQuery, String> {
        StudentQuery::try_from(value.clone())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageNo(pub u32);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageSize(pub u16);

impl PageNo {
    pub fn new(value: Option<u32>) -> Result<Self, String> {
        match value {
            Some(v) => Ok(PageNo(v)),
            None => Ok(PageNo(1))
        }
    }
}

impl PageSize {
    pub fn new(value: Option<u16>) -> Result<Self, String> {
        match value {
            Some(v) => Ok(PageSize(v)),
            None => Ok(PageSize(10))
        }
    }
}