use crate::api::cmd::IdCommand;
use crate::api::primitive::students::{Address, Age, ClassId, StuNo, UserName};
use serde::{Deserialize, Serialize};


use crate::api::cmd::student_cmd::{StudentCreateCommand, StudentPageQueryCommand, StudentUpdateCommand};
use crate::api::primitive::students::{ClassIdQuery, StuNoQuery, UserNameQuery};
use crate::ddd::core::{DomainModel, DomainPrimitive, Safes};
use crate::ddd::core::{Id, IdOper};

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

impl Safes for StudentCreate {}

impl DomainModel for StudentCreate {
    type CQES = StudentCreateCommand;
    fn new(value: &Self::CQES) -> Result<Self, String> {
        StudentCreate::try_from(value.to_owned())
    }
}

impl TryFrom<StudentCreateCommand> for StudentCreate {
    type Error = String;
    fn try_from(value: StudentCreateCommand) -> Result<Self, Self::Error> {
        Ok(Self {
            stu_no: StuNo::new(value.stu_no)?,
            name: UserName::new(value.name)?,
            age: Age::new(value.age)?,
            class_id: ClassId::new(value.class_id)?,
            address: Address::new(value.address)?,
        })
    }
}



impl DomainModel for IdOper<i64> {

    type CQES = IdCommand<i64>;
    fn new(value: &Self::CQES) -> Result<IdOper<i64>, String> {
        IdOper::<i64>::try_from(value.to_owned())
    }
}


impl TryFrom<IdCommand<i64>> for IdOper<i64> {
    type Error = String;
    fn try_from(value: IdCommand<i64>) -> Result<Self, Self::Error> {
        Ok(IdOper {  id: Id::new(value.id)? })
    }
}






#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudentQuery {
    // pub page_no: PageNo,
    // pub page_size: PageSize,
    pub stu_no: StuNoQuery,
    pub name: UserNameQuery,
    pub class_id: ClassIdQuery,
}

unsafe impl Send for StudentQuery {}
unsafe impl Sync for StudentQuery {}
impl Safes for StudentQuery {}

impl DomainModel for StudentQuery {
    type CQES = StudentPageQueryCommand;

    fn new(value: &Self::CQES) -> Result<Self, String> {
        StudentQuery::try_from(value.to_owned())
    }
}

impl TryFrom<StudentPageQueryCommand> for StudentQuery {
    type Error = String;
    fn try_from(value: StudentPageQueryCommand) -> Result<Self, Self::Error> {
        Ok(StudentQuery {
            stu_no: StuNoQuery::new(value.stu_no)?,
            name: UserNameQuery::new(value.name)?,
            class_id: ClassIdQuery::new(value.class_id)?
        })
    }
}






#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudentUpdate {
    pub id: Id<i64>,
    // pub stu_no: StuNo,
    pub name: UserName,
    pub age: Age,
    pub class_id: ClassId,
    pub address: Address,
}

unsafe impl Send for StudentUpdate {}
unsafe impl Sync for StudentUpdate {}

impl Safes for StudentUpdate {}

impl DomainModel for StudentUpdate {

    type CQES = StudentUpdateCommand;
    fn new(value: &Self::CQES) -> Result<StudentUpdate, String> {
        StudentUpdate::try_from(value.clone())
    }
}

impl TryFrom<StudentUpdateCommand> for StudentUpdate {
    type Error = String;
    fn try_from(value: StudentUpdateCommand) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Id::new(value.id)?,
            name: UserName::new(value.name)?,
            age: Age::new(value.age)?,
            class_id: ClassId::new(value.class_id)?,
            address: Address::new(value.address)?,
        })
    }
}

