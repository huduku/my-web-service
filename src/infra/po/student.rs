use rbatis::htmlsql_select_page;
use serde::{Deserialize, Serialize};
use crate::ddd::core::{DomainPrimitive, Id};
use crate::domain::entity::student::Student;
use crate::api::primitive::students::{Address, Age, ClassId, StuNo, UserName};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudentPO {
    pub id: Option<i64>,
    pub stu_no: Option<String>,
    pub name: Option<String>,
    pub age: Option<u16>,
    pub class_id: Option<u32>,
    pub address: Option<String>,
}




rbatis::crud!(StudentPO {});
rbatis::impl_select!(StudentPO{select_by_id(id: i64) -> Option => "`where id = #{id} limit 1`"});
rbatis::impl_select!(StudentPO{select_by_stu_no(stu_no: String) -> Option => "`where stu_no = #{stu_no} limit 1`"});

impl StudentPO {
    htmlsql_select_page!(select_page(dto:&Option<StudentPO>) -> StudentPO => "src/resources/mapper/student.html");
}


impl TryFrom<StudentPO> for Student {
    type Error = String;

    fn try_from(value: StudentPO) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Id::new(value.id)?,
            stu_no: StuNo::new(value.stu_no)?,
            name: UserName::new(value.name)?,
            age: Age::new(value.age)?,
            class_id: ClassId::new(value.class_id)?,
            address: Address::new(value.address)?
        })
    }
}
