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

    rb.get_intercept::<LogInterceptor>()
        .expect("rbatis LogInterceptor init fail!")
        .set_level_filter(LevelFilter::Debug);

    rb
}

pub struct DbResult<T>(pub Result<T, String>);

impl<T> From<Result<T, rbatis::Error>> for DbResult<T> {
    fn from(value: Result<T, rbatis::Error>) -> Self {
        DbResult(value.map_err(|_| "数据库异常".to_string()))
    }
}