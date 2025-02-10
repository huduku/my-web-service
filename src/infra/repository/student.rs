use crate::app::dto::res::{DbRes, PageRes};
use crate::domain::cmd::student_cmd::StudentPageQuery;
use crate::domain::core::{DomainModel, Id, PageQuery};
use crate::domain::entity::student::Student;
use crate::domain::primitive::students::StuNo;
use crate::domain::repo::student::StudentRepository;
use crate::domain::repo::Repository;
use crate::infra::po::student::StudentPO;
use crate::pool;

pub struct StudentRepositoryImpl;

impl Repository<Id<i64>, Student> for StudentRepositoryImpl {
    async fn attach(aggr: Student) {
        todo!()
    }

    async fn detach(aggr: Student) {
        todo!()
    }

    async fn find(id: Id<i64>) -> Result<Student, String> {
        todo!()
    }

    async fn save(aggr: Student) {
        todo!()
    }

    async fn remove(aggr: Student) {
        todo!()
    }
}

impl StudentRepository for StudentRepositoryImpl {
    async fn find_one(stu_no: StuNo) -> Result<Student, String> {
        let DbRes(res)  = StudentPO::select_by_stu_no(pool!(), stu_no.0).await.into();
        match res {
            Ok(stu) => {
                match stu {
                    Some(student) => Student::new(&student),
                    None => Err("查询数据为空".to_string())
                }
            },
            Err(e) => Err(e)
        }
    }

    async fn find_page(query: PageQuery<StudentPageQuery>) -> Result<PageRes<Student>, String> {
        todo!()
    }

    async fn count(query: StudentPageQuery) -> Result<u64, String> {
        todo!()
    }
}