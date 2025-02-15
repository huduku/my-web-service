use crate::api::primitive::students::StuNo;
use crate::api::res::PageRes;
use crate::ddd::core::{Id, PageQuery};
use crate::ddd::repo::Repository;
use crate::app::cmd::student_cmd::StudentQuery;
use crate::domain::entity::student::Student;

pub trait StudentRepository : Repository<RawId=i64, Id=Id<i64>, Aggr=Student> {
    
    async fn find_one(&self, stu_no: StuNo) -> Result<Student, String>;
    
    async fn find_page(&self, page_query: PageQuery<StudentQuery>) -> Result<PageRes<Student>, String>;
    
    async fn count(&self, query: StudentQuery) -> Result<u64, String>;
    
}
