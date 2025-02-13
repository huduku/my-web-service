use crate::domain::service::student_services::StudentService;
use crate::infra::repository::student::StudentRepositoryImpl;

pub struct AppStudentSrv {
    pub student_service: StudentService<StudentRepositoryImpl>
}

impl AppStudentSrv {
    pub fn new() -> Self {
        Self {
            student_service: StudentService::new(StudentRepositoryImpl{})
        }
    }
}