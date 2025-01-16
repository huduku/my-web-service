use rbatis::{htmlsql_select_page, impl_select_page, PageRequest};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Student {
    pub id: Option<i64>,
    pub stu_no: Option<String>,
    pub name: Option<String>,
    pub age: Option<i8>,
    pub class_id: Option<i64>,
    pub address: Option<String>,
}

rbatis::crud!(Student{});


impl Student {
    htmlsql_select_page!(select_page(dto:&Option<Student>) -> Student => "src/resources/mapper/student.html");
}

