use std::sync::LazyLock;
use log::LevelFilter;
use rbatis::intercept_log::LogInterceptor;
use rbatis::RBatis;
use crate::app::srv::container::AppSrvContainer;


pub static CONTEXT: LazyLock<ServiceContext> = LazyLock::new(ServiceContext::default);

#[macro_export]
macro_rules! pool {
    () => {
        &$crate::context::CONTEXT.rb
    };
}


pub struct ServiceContext {
    pub rb: RBatis,
    pub app_srv_container: AppSrvContainer
}

impl ServiceContext {
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

impl Default for ServiceContext {
    fn default() -> Self {
        ServiceContext {
            rb: RBatis::new(),
            app_srv_container: AppSrvContainer::default(),
        }
    }
}