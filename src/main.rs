use axum::Router;
use fast_log::plugin::file_split::{DateType, KeepType, Rolling, RollingType};
use fast_log::Config;
use tower_http::cors::{Any, CorsLayer};
use my_web_service::route::routes::student_routes;
use my_web_service::context::CONTEXT;

#[tokio::main]
async fn main() {
    init_log();
    CONTEXT.init_db().await;
    // 初始化数据库
    // let rb = init_db().await;
    // let shared_states = Arc::new(AppState { rbatis: rb.clone() });
    // 定义路由
    let app= Router::new()
        .nest("/api", Router::new().merge(student_routes()))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        );

    // 启动服务
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn init_log() {
    //init fast log
    let mut cfg = Config::new()
        .chan_len(Some(100000))
        .level(log::LevelFilter::Debug)
        .file_split(
            "target/logs/",
            Rolling::new(RollingType::ByDate(DateType::Day)),
            KeepType::KeepNum(120),
            fast_log::plugin::packer::LogPacker {},
        );
    cfg = cfg.console();
    let _ = fast_log::init(cfg);
}
