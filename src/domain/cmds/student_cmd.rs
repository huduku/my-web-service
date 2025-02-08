use serde::{Deserialize, Serialize};

use crate::domain::{
    po::student::Student,
    primitives::{
        students::{Address, Age, ClassId, StuNo, UserName}
    }
};

use crate::domain::core::{DomainGuard, DomainPrimitive};
use crate::domain::primitives::{PageNo, PageSize, Id, IdOper};
use crate::domain::primitives::students::{ClassIdQuery, StuNoQuery, UserNameQuery};
use crate::dto::req::PageReq;

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

impl DomainGuard<StudentCreate> for Student {
    fn new(value: Self) -> Result<StudentCreate, String> {
        StudentCreate::try_from(value)
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
        }
    }
}




impl DomainGuard<IdOper<i64>> for Student {
    fn new(value: Self) -> Result<IdOper<i64>, String> {
        IdOper::<i64>::try_from(value)
    }
}


impl TryFrom<Student> for IdOper<i64> {
    type Error = String;
    fn try_from(value: Student) -> Result<Self, Self::Error> {
        let id = Id::new(value.id)?;
        Ok(IdOper {
            id
        })
    }
}

impl From<IdOper<i64>> for Student {

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

impl DomainGuard<StudentQuery> for PageReq<Student> {
    fn new(value: Self) -> Result<StudentQuery, String> {
        StudentQuery::try_from(value)
    }
}


impl TryFrom<PageReq<Student>> for StudentQuery {
    type Error = String;
    fn try_from(value: PageReq<Student>) -> Result<Self, Self::Error> {

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

impl From<StudentQuery> for PageReq<Student> {

    fn from(value: StudentQuery) -> Self {
        Self {
            page_no: Some(value.page_no.0),
            page_size: Some(value.page_size.0),
            req: Some(Student {
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


impl DomainGuard<StudentUpdate> for Student {
    fn new(value: Self) -> Result<StudentUpdate, String> {
        StudentUpdate::try_from(value.clone())
    }
}




impl TryFrom<Student> for StudentUpdate {
    type Error = String;
    fn try_from(value: Student) -> Result<Self, Self::Error> {

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


impl From<StudentUpdate> for Student {

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
