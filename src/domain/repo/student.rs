use crate::api::res::PageRes;
use crate::domain::cmd::student_cmd::StudentPageQuery;
use crate::domain::core::{Id, PageQuery};
use crate::domain::entity::student::Student;
use crate::api::primitive::students::StuNo;
use crate::domain::repo::Repository;


pub trait StudentRepository : Repository<Id<i64>, Student> {
    
    async fn find_one(stu_no: StuNo) -> Result<Student, String>;
    
    async fn find_page(query: PageQuery<StudentPageQuery>) -> Result<PageRes<Student>, String>;
    
    async fn count(query: StudentPageQuery) -> Result<u64, String>;
    
}