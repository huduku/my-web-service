use serde::{Deserialize, Serialize};

use crate::domain::primitive::students::{Address, Age, ClassId, StuNo, UserName};

use crate::domain::core::{DomainModel, DomainPrimitive, PageQuery};
use crate::domain::core::{Id, IdOper, PageNo, PageSize};
use crate::domain::primitive::students::{ClassIdQuery, StuNoQuery, UserNameQuery};
use crate::app::dto::req::PageReq;
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
    type CQES = StudentPO;
    fn new(value: &Self::CQES) -> Result<Self, String> {
        StudentCreate::try_from(value.to_owned())
    }
}

impl TryFrom<StudentPO> for StudentCreate {
    type Error = String;
    fn try_from(value: StudentPO) -> Result<Self, Self::Error> {

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

    type CQES = StudentPO;
    fn new(value: &Self::CQES) -> Result<IdOper<i64>, String> {
        IdOper::<i64>::try_from(value.to_owned())
    }
}


impl TryFrom<StudentPO> for IdOper<i64> {
    type Error = String;
    fn try_from(value: StudentPO) -> Result<Self, Self::Error> {
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
pub struct StudentQuery {
    pub page_no: PageNo,
    pub page_size: PageSize,
    pub stu_no: StuNoQuery,
    pub name: UserNameQuery,
    pub class_id: ClassIdQuery,
}

unsafe impl Send for StudentQuery {}
unsafe impl Sync for StudentQuery {}

impl DomainModel for StudentQuery {
    type CQES = PageReq<StudentPO>;
    fn new(value: &Self::CQES) -> Result<StudentQuery, String> {
        StudentQuery::try_from(value.to_owned())
    }
}


impl TryFrom<PageReq<StudentPO>> for StudentQuery {
    type Error = String;
    fn try_from(value: PageReq<StudentPO>) -> Result<Self, Self::Error> {

        let page_no = PageNo::new(value.page_no)?;
        let page_size=  PageSize::new(value.page_size)?;

        let stu_no;
        let name: UserNameQuery;
        let class_id;

        match value.req {
            Some(v) => {
                stu_no = StuNoQuery::new(v.stu_no)?;
                name = UserNameQuery::new(v.name)?;
                class_id = ClassIdQuery::new(v.class_id)?;
            },
            None=> {
                stu_no = StuNoQuery(None);
                name = UserNameQuery(None);
                class_id = ClassIdQuery(None);
            }
        }


        Ok(Self {
            page_no,
            page_size,
            stu_no,
            name,
            class_id,
        })
    }
}

impl From<StudentQuery> for PageReq<StudentPO> {

    fn from(value: StudentQuery) -> Self {
        Self {
            page_no: Some(value.page_no.0),
            page_size: Some(value.page_size.0),
            req: Some(StudentPO {
                id: None,
                stu_no: value.stu_no.0,
                name: value.name.0,
                age: None,
                class_id: value.class_id.0,
                address: None,
            })
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

    type CQES = StudentPO;
    fn new(value: &Self::CQES) -> Result<StudentUpdate, String> {
        StudentUpdate::try_from(value.clone())
    }
}




impl TryFrom<StudentPO> for StudentUpdate {
    type Error = String;
    fn try_from(value: StudentPO) -> Result<Self, Self::Error> {

        // let id = Id::new(value.id)?;
        // let name =  UserName::new(value.name)?;
        // let age =  Age::new(value.age)?;
        // let class_id =  ClassId::new(value.class_id)?;
        // let address = Address::new(value.address)?;
        
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


impl DomainModel for PageQuery<StudentQuery> {

    type CQES = PageReq<StudentPO>;
    
    fn new(value: &Self::CQES) -> Result<Self, String> {
        PageQuery::<StudentQuery>::try_from(value.to_owned())
    }
    
}

impl TryFrom<PageReq<StudentPO>> for PageQuery<StudentQuery> {
    type Error = String;

    fn try_from(value: PageReq<StudentPO>) -> Result<Self, Self::Error> {
        todo!()
    }
}