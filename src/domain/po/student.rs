use std::str::FromStr;
use chrono::{DateTime, NaiveDateTime, Utc};
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
    // #[serde(serialize_with = "serialize_date")]
    pub created_at: Option<NaiveDateTime>,
}




rbatis::crud!(Student {});
rbatis::impl_select!(Student{select_by_id(id: i64) -> Option => "`where id = #{id} limit 1`"});

impl Student {
    htmlsql_select_page!(select_page(dto:&Option<Student>) -> Student => "src/resources/mapper/student.html");
}


#[test]
fn it_works() {
    let stu = Student {
        id: Some(1),
        stu_no:  Some("1234".to_string()),
        name: Some("1234".to_string()),
        age: Some(12),
        class_id: Some(34534),
        address: Some("1234".to_string()),
        // #[serde(serialize_with = "serialize_date")]
        created_at: Some(Utc::now().naive_local()),
    };
    println!("{:#?}", stu);
}