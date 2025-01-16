use log::LevelFilter;
use rbatis::intercept_log::LogInterceptor;
use rbatis::rbatis::RBatis;

pub async fn init_db() -> RBatis {

    let rb = RBatis::new();
    rb.init(
        rbdc_mysql::driver::MysqlDriver {},
        "mysql://root:123456@127.0.0.1:3306/test",
    )
        .unwrap();

    rb.get_intercept::<LogInterceptor>().expect("rbatis LogInterceptor init fail!").set_level_filter(LevelFilter::Debug);

    rb
}