
use crate::api::cmd::IdCommand;
use crate::api::res::PageRes;
use crate::ddd::core::{DomainPrimitive, Id, PageQuery};
use crate::app::cmd::student_cmd::StudentQuery;
use crate::domain::entity::student::Student;
use crate::domain::repo::student::StudentRepository;

pub struct StudentService<T: StudentRepository + 'static> {
    pub(crate) student_repository : &'static T
}

impl<T: StudentRepository> StudentService<T> {

    pub fn new(student_repository: &'static T) -> Self {
        Self {
            student_repository
        }
    }

    pub async fn get_student(&self, id: IdCommand<i64>) -> Result<Student, String> { // T::ID
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
