use std::sync::LazyLock;
use log::LevelFilter;
use rbatis::intercept_log::LogInterceptor;
use rbatis::RBatis;
use crate::ddd::core::Id;
use crate::domain::entity::student::Student;
use crate::domain::repo::student::StudentRepository;
use crate::domain::service::student_services::StudentService;
use crate::infra::repository::student::StudentRepositoryImpl;

pub static CONTEXT: LazyLock<ServiceContext<StudentRepositoryImpl>> = LazyLock::new(|| ServiceContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &$crate::context::CONTEXT.rb
    };
}


pub struct ServiceContext<T: StudentRepository<ID=Id<i64>, Aggr=Student>> {
    pub rb: RBatis,
    pub student_service: StudentService<T>
}

impl<T: StudentRepository<ID=Id<i64>, Aggr=Student>> ServiceContext<T> {
    pub async fn init_db(&self) {
        self.rb.init(
            rbdc_mysql::driver::MysqlDriver {},
            "mysql://root:123456@127.0.0.1:3306/test",
        )
            .unwrap();

        self.rb.get_intercept::<LogInterceptor>()
            .expect("rbatis LogInterceptor init fail!")
            .set_level_filter(LevelFilter::Debug);

    }
}

impl Default for ServiceContext<StudentRepositoryImpl> {
    fn default() -> Self {
        ServiceContext {
            rb: {
                let rb = RBatis::new();
                rb
            },
            student_service: StudentService::new(StudentRepositoryImpl{})
        }
    }
}