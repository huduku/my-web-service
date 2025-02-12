use crate::api::res::PageRes;
use crate::domain::cmd::student_cmd::StudentQuery;
use crate::ddd::core::{Id, PageQuery};
use crate::domain::entity::student::Student;
use crate::api::primitive::students::StuNo;
use crate::ddd::repo::Repository;

pub trait StudentRepository : Repository {
    
    async fn find_one(&self, stu_no: StuNo) -> Result<Student, String>;
    
    async fn find_page(&self, page_query: PageQuery<StudentQuery>) -> Result<PageRes<Student>, String>;
    
    async fn count(&self, query: StudentQuery) -> Result<u64, String>;
    
}