use rbatis::htmlsql_select_page;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Student {
    pub id: Option<i64>,
    pub stu_no: Option<String>,
    pub name: Option<String>,
    pub age: Option<u16>,
    pub class_id: Option<u32>,
    pub address: Option<String>,
}

rbatis::crud!(Student {});
rbatis::impl_select!(Student{select_by_id(id: i64) -> Option => "`where id = #{id} limit 1`"});

impl Student {
    htmlsql_select_page!(select_page(dto:&Option<Student>) -> Student => "src/resources/mapper/student.html");
}
