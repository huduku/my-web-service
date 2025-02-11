use serde::{Deserialize, Serialize};
use crate::app::dto::IdCommand;
use crate::domain::primitive::students::{Address, Age, ClassId, StuNo, UserName};

use crate::app::dto::req::PageReq;
use crate::app::dto::student_cmd::{StudentCreateCommand, StudentPageQueryCommand, StudentUpdateCommand};
use crate::domain::core::{DomainModel, DomainPrimitive, PageQuery};
use crate::domain::core::{Id, IdOper};
use crate::domain::primitive::students::{ClassIdQuery, StuNoQuery, UserNameQuery};
use crate::infra::po::student::StudentPO;

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

impl DomainModel for StudentCreate {
    type CQES = StudentCreateCommand;
    fn new(value: &Self::CQES) -> Result<Self, String> {
        StudentCreate::try_from(value.to_owned())
    }
}

impl TryFrom<StudentCreateCommand> for StudentCreate {
    type Error = String;
    fn try_from(value: StudentCreateCommand) -> Result<Self, Self::Error> {

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

impl From<StudentCreate> for StudentPO {

    fn from(value: StudentCreate) -> Self {
        Self {
            id: None,
            stu_no: Some(value.stu_no.0),
            name: Some(value.name.0),
            age: Some(value.age.0),
            class_id: Some(value.class_id.0),
            address: Some(value.address.0),
        }
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
        let id = Id::new(value.id)?;
        Ok(IdOper {
            id
        })
    }
}

impl From<IdOper<i64>> for StudentPO {

    fn from(value: IdOper<i64>) -> Self {
        Self {
            id: Some(value.id.0),
            stu_no: None,
            name: None,
            age: None,
            class_id: None,
            address: None,
        }
    }
}





#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudentPageQuery {
    // pub page_no: PageNo,
    // pub page_size: PageSize,
    pub stu_no: StuNoQuery,
    pub name: UserNameQuery,
    pub class_id: ClassIdQuery,
}

unsafe impl Send for StudentPageQuery {}
unsafe impl Sync for StudentPageQuery {}

impl DomainModel for StudentPageQuery {
    type CQES = StudentPageQueryCommand;

    fn new(value: &Self::CQES) -> Result<Self, String> {
        StudentPageQuery::try_from(value.to_owned())
    }
}

impl TryFrom<StudentPageQueryCommand> for StudentPageQuery {
    type Error = String;
    fn try_from(value: StudentPageQueryCommand) -> Result<Self, Self::Error> {
        let stu_no = StuNoQuery::new(value.stu_no)?;
        let name = UserNameQuery::new(value.name)?;
        let class_id = ClassIdQuery::new(value.class_id)?;

        Ok(StudentPageQuery {
            stu_no,
            name,
            class_id
        })
    }
}


impl From<PageQuery<StudentPageQuery>> for PageReq<StudentPO> {

    fn from(value: PageQuery<StudentPageQuery>) -> Self {
        match value.query {
            Some(q) => Self {
                page_no: Some(value.page_no.0),
                page_size: Some(value.page_size.0),
                req: Some(StudentPO {
                    id: None,
                    stu_no: q.stu_no.0,
                    name: q.name.0,
                    age: None,
                    class_id: q.class_id.0,
                    address: None,
                })
            },
            None => Self {
                page_no: Some(value.page_no.0),
                page_size: Some(value.page_size.0),
                req: None
            }
        }
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


impl DomainModel for StudentUpdate {

    type CQES = StudentUpdateCommand;
    fn new(value: &Self::CQES) -> Result<StudentUpdate, String> {
        StudentUpdate::try_from(value.clone())
    }
}




impl TryFrom<StudentUpdateCommand> for StudentUpdate {
    type Error = String;
    fn try_from(value: StudentUpdateCommand) -> Result<Self, Self::Error> {

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


impl From<StudentUpdate> for StudentPO {
    fn from(value: StudentUpdate) -> Self {
        Self {
            id: Some(value.id.0),
            stu_no: None,// 唯一索引， 不能更新此字段
            name: Some(value.name.0),
            age: Some(value.age.0),
            class_id: Some(value.class_id.0),
            address: Some(value.address.0),
        }
    }
}