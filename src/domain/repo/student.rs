use crate::api::res::PageRes;
use crate::domain::cmd::student_cmd::StudentQuery;
use crate::ddd::core::{Aggregate, DomainModel, DomainPrimitive, Id, Identifier, PageQuery};
use crate::domain::entity::student::Student;
use crate::api::primitive::students::StuNo;
use crate::ddd::repo::Repository;

pub trait StudentRepository : Repository {

    // type ID : DomainPrimitive<i64> + Identifier;
    // type Aggr : DomainModel + Aggregate<Self::ID>;
    
    async fn find_one(&self, stu_no: StuNo) -> Result<Student, String>;
    
    async fn find_page(&self, page_query: PageQuery<StudentQuery>) -> Result<PageRes<Student>, String>;
    
    async fn count(&self, query: StudentQuery) -> Result<u64, String>;
    
}


pub trait StudentRepo : StudentRepository<ID=Id<i64>, Aggr=Student> {}