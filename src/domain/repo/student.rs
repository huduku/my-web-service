use crate::app::dto::res::PageRes;
use crate::domain::cmd::student_cmd::StudentQuery;
use crate::domain::core::{Id, PageQuery};
use crate::domain::entity::student::Student;
use crate::domain::primitive::students::StuNo;
use crate::domain::repo::Repository;


pub trait StudentRepository : Repository<Id<i64>, Student> {
    
    async fn find_one(stu_no: StuNo) -> Result<Student, String>;
    
    async fn find_page(query: PageQuery<StudentQuery>) -> Result<PageRes<Student>, String>;
    
    async fn count(query: StudentQuery) -> Result<u64, String>;
    
}