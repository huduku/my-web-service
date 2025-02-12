use crate::infra::data::student::StudentDO;

use crate::api::cmd::IdCommand;
use crate::api::res::PageRes;
use crate::ddd::core::{DomainPrimitive, Id, PageQuery};
use crate::ddd::repo::Repository;
use crate::domain::cmd::student_cmd::StudentQuery;
use crate::domain::entity::student::Student;
use crate::domain::repo::student::StudentRepository;
use crate::infra::repository::student::StudentRepositoryImpl;
use crate::infra::repository::DbRes;
use rbatis::rbdc::db::ExecResult;
use crate::pool;

pub struct StudentService<'a> {
    pub student_repository : StudentRepositoryImpl<'a>,
}

impl StudentService<'_> {

    pub fn new() -> Self {
        Self {
            student_repository: StudentRepositoryImpl::new()
        }
    }

    pub async fn get_student(&self, id: IdCommand<i64>) -> Result<Student, String> {
        self.student_repository.find(Id::new(id.id)?).await
    }

    pub async fn create_student(student: StudentDO) -> Result<ExecResult, String> {
        let DbRes(res) = StudentDO::insert(pool!(), &student).await.into();
        res
    }

    pub async fn update_student(student: StudentDO) -> Result<ExecResult, String> {
        let DbRes(res) = StudentDO::update_by_column(pool!(), &student, "id").await.into();
        res
    }

    pub async fn delete_student(id: i64) -> Result<ExecResult, String> {
        let DbRes(res) = StudentDO::delete_by_column(pool!(), "id", id).await.into();
        res
    }

    pub async fn list_students(&self, page_query: PageQuery<StudentQuery>,)
                               -> Result<PageRes<Student>, String> {
        self.student_repository.find_page(page_query).await
    }
}
