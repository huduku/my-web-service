use serde::{Deserialize, Serialize};
use crate::domain::model::student::Student;

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


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudentUpdate {
    pub id: Id,
    // pub stu_no: StuNo,
    pub name: UserName,
    pub age: Age,
    pub class_id: ClassId,
    pub address: Address,
}

unsafe impl Send for StudentUpdate {}
unsafe impl Sync for StudentUpdate {}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Id(i64);


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StuNo(String);


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserName(String);


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Age(u8);


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClassId(u32);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address(String);

impl Id {
    fn new(value: Option<i64>) -> Result<Self, String> {
        match value {
            Some(v) =>  Ok(Id(v)),
            None => Err("id 不能为空".to_string())
        }
    }
}

impl StuNo {
    fn new(value: Option<String>) -> Result<Self, String> {
        match value {
            Some(v) => {
                if v.is_empty() {
                    return Err("stu_no 不能为空值".to_string());
                }
                if v.len() < 8 {
                    return Err("stu_no 长度不能小于 8".to_string());
                }
                if !v.chars().all(|c| c.is_ascii_alphanumeric()) {
                    return Err("stu_no 只能由字母和数字组成".to_string());
                }
                Ok(StuNo(v))
            },
            None => Err("stu_no 不能为空".to_string())
        }
    }
}




impl UserName {
    fn new(value: Option<String>) -> Result<Self, String> {
        match value {
            Some(v) => Ok(UserName(v)),
            None => Err("name 不能为空".to_string())
        }
    }
}


impl Age {
    fn new(value: Option<u8>) -> Result<Self, String> {
        match value {
            Some(v) => {
                if v < 0 {
                    return Err("age 不能小于 0".to_string());
                }
                if v > 127 {
                    return Err("age 不能大于 127".to_string());
                }
                Ok(Age(v))
            },
            None => Err("age 不能为空".to_string())
        }
    }
}

impl ClassId {
    fn new(value: Option<u32>) -> Result<Self, String> {
        match value {
            Some(v) => Ok(ClassId(v)),
            None => Err("class_id 不能为空".to_string())
        }
    }
}

impl Address {
    fn new(value: Option<String>) -> Result<Self, String> {
        match value {
            Some(v) => Ok(Address(v)),
            None => Err("address 不能为空".to_string())
        }
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


impl TryFrom<Student> for StudentUpdate {
    type Error = String;
    fn try_from(value: Student) -> Result<Self, Self::Error> {

        let id = Id::new(value.id)?;
        // let stu_no = ImmuteStuNo::new(value.stu_no)?;
        let name =  UserName::new(value.name)?;
        let age =  Age::new(value.age)?;
        let class_id =  ClassId::new(value.class_id)?;
        let address = Address::new(value.address)?;

        Ok(Self {
            id,
            // stu_no: ,
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
            address: Some(value.address.0)
        }
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
            address: Some(value.address.0)
        }
    }
}

#[test]
fn test() {
    let stu = Student {
        id: Some(0),
        stu_no: Some("10247015".to_string()),
        name: Some("张三".to_string()),
        age: Some(180),
        class_id: Some(5),
        address:  Some("学号".to_string())
    };

    let stu = StudentCreate::try_from(stu);
    if stu.is_ok() {
        println!("ok: {}", serde_json::to_string(&stu.unwrap()).unwrap());
    } else {
        println!("err: {}", &stu.err().unwrap());
    }

}