
use crate::infra::repository::student::StudentRepositoryImpl;

pub struct AppRepoContext {
    pub student_repository: &'static StudentRepositoryImpl
}

impl Default for AppRepoContext {
    fn default() -> Self {
        Self {
            student_repository: &StudentRepositoryImpl{}
        }
    }
}