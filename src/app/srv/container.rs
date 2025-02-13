use crate::domain::service::student_services::StudentService;
use crate::infra::repository::student::StudentRepositoryImpl;

pub struct AppSrvContainer {
    pub student_service: StudentService<StudentRepositoryImpl>
}

impl Default for AppSrvContainer {
    fn default() -> Self {
        Self {
            student_service: StudentService::new(StudentRepositoryImpl{})
        }
    }
}