use crate::app::context::repo::AppRepoContext;
use crate::domain::service::student_services::StudentService;
use crate::infra::repository::student::StudentRepositoryImpl;

pub struct AppSrvContainer {
    pub student_service: StudentService<StudentRepositoryImpl>
}

impl Default for AppSrvContainer {
    fn default() -> Self {
        let app_repo_context = &AppRepoContext::default();
        Self {
            student_service: StudentService::new(app_repo_context.student_repository)
        }
    }
}