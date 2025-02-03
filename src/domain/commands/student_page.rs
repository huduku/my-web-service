use serde::{Deserialize, Serialize};

use crate::{domain::{
    po::student::Student, primitives::{
        dp::{DomainPrimitive, PageNo, PageSize}, 
        students::{ClassIdQuery, StuNoQuery, UserNameQuery}
    }
}, dto::req::PageReq};



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

impl DomainPrimitive<StudentQuery> for PageReq<Student> {
    fn new(value: &Self) -> Result<StudentQuery, String> {
        StudentQuery::try_from(value.clone())
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