extern crate core;

mod models;
mod db;
mod services;
mod routes;
mod res;

mod error;
mod req;

use axum::Router;
use db::init_db;
use fast_log::plugin::file_split::{DateType, KeepType, Rolling, RollingType};
use fast_log::Config;
use rbatis::rbatis::RBatis;
use routes::student_routes;
use std::sync::Arc;


pub struct AppState {
    pub rbatis: RBatis,
}
#[tokio::main]
async fn main() {
    init_log();
    // 初始化数据库
    let rb = init_db().await;
    
    
    let shared_states = Arc::new(AppState {
        rbatis: rb.clone()
    });

    // 定义路由
    let app = Router::new().nest(
        "/api",
        Router::new().
        merge(student_routes())
    ).with_state(shared_states);
    
    // 启动服务
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn init_log() {
    //init fast log
    let mut cfg = Config::new()
        .chan_len(Some(100000))
        .level(log::LevelFilter::Debug)
        .file_split("target/logs/",
                    Rolling::new(RollingType::ByDate(DateType::Day)),
                    KeepType::KeepNum(120),
                    fast_log::plugin::packer::LogPacker {},
        );
    cfg = cfg.console();
    let _ = fast_log::init(cfg);
   
}