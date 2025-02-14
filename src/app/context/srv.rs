use crate::context::CONTEXT;
use crate::domain::service::student_services::StudentService;
use crate::infra::repository::student::StudentRepositoryImpl;

pub struct AppSrvContainer {
    pub student_service: StudentService<StudentRepositoryImpl>
}

impl Default for AppSrvContainer {
    fn default() -> Self {
        let app_repo_context = &CONTEXT.app_repo_context;
        Self {
            student_service: StudentService::new(app_repo_context.student_repository),
        }
    }
}