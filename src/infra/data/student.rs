use rbatis::htmlsql_select_page;
use serde::{Deserialize, Serialize};
use crate::ddd::core::{DomainModel, DomainPrimitive, Id, IdOper};
use crate::domain::entity::student::Student;
use crate::api::primitive::students::{Address, Age, ClassId, StuNo, UserName};
use crate::domain::cmd::student_cmd::{StudentCreate, StudentQuery, StudentUpdate};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudentDO {
    pub id: Option<i64>,
    pub stu_no: Option<String>,
    pub name: Option<String>,
    pub age: Option<u16>,
    pub class_id: Option<u32>,
    pub address: Option<String>,
}

rbatis::crud!(StudentDO {});
rbatis::impl_select!(StudentDO{select_by_id(id: i64) -> Option => "`where id = #{id} limit 1`"});
rbatis::impl_select!(StudentDO{select_by_stu_no(stu_no: String) -> Option => "`where stu_no = #{stu_no} limit 1`"});

impl StudentDO {
    htmlsql_select_page!(select_page(dto:&Option<StudentDO>) -> StudentDO => "src/resources/mapper/student.html");
}


impl From<StudentCreate> for StudentDO {

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



impl From<IdOper<i64>> for StudentDO {

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


impl From<StudentQuery> for StudentDO {
    fn from(value: StudentQuery) -> Self {
        Self {
            id: None,
            stu_no: value.stu_no.0,
            name: value.name.0,
            age: None,
            class_id: value.class_id.0,
            address: None,
        }
    }
}



impl From<StudentUpdate> for StudentDO {
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

impl DomainModel for Student {
    type CQES = StudentDO;
    fn new(value: &Self::CQES) -> Result<Self, String> {
        Self::try_from(value.to_owned())
    }
}

impl TryFrom<StudentDO> for Student {
    type Error = String;

    fn try_from(value: StudentDO) -> Result<Self, Self::Error> {
        Ok(Self {
            id: match value.id { 
                Some(id)=> Some(Id(id)),
                None => None
            },
            stu_no: Some(StuNo::new(value.stu_no)?),
            name: UserName::new(value.name)?,
            age: Age::new(value.age)?,
            class_id: ClassId::new(value.class_id)?,
            address: Address::new(value.address)?
        })
    }
}

impl From<Student> for StudentDO {
    fn from(value: Student) -> Self {
        Self {
            id: match value.id { Some(v)=> Some(v.0), None=> None },
            stu_no: match value.stu_no { Some(v)=> Some(v.0), None=> None },// 唯一索引， 不能更新此字段
            name: Some(value.name.0),
            age: Some(value.age.0),
            class_id: Some(value.class_id.0),
            address: Some(value.address.0),
        }
    }
}



