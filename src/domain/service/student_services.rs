
use crate::api::cmd::IdCommand;
use crate::api::res::PageRes;
use crate::ddd::core::{DomainPrimitive, Id, PageQuery};
use crate::ddd::repo::Repository;
use crate::domain::cmd::student_cmd::StudentQuery;
use crate::domain::entity::student::Student;
use crate::domain::repo::student::StudentRepository;
use crate::infra::repository::student::StudentRepositoryImpl;

pub struct StudentService {
    pub(crate) student_repository : StudentRepositoryImpl
}

impl StudentService {

    pub fn new(student_repository: StudentRepositoryImpl) -> Self {
        Self {
            student_repository
        }
    }

    pub async fn get_student(&self, id: IdCommand<i64>) -> Result<Student, String> {
        self.student_repository.find(Id::new(id.id)?).await
    }
    

    pub async fn save_student(&self, student: Student) -> Result<(), String> {
        self.student_repository.save(student).await
    }

    pub async fn delete_student(&self, id: IdCommand<i64>) -> Result<(), String> {
        self.student_repository.remove(Id::new(id.id)?).await
    }

    pub async fn list_students(&self, page_query: PageQuery<StudentQuery>,)
                               -> Result<PageRes<Student>, String> {
        self.student_repository.find_page(page_query).await
    }
}
