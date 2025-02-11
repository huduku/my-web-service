use rbatis::RBatis;

pub mod ddd;

pub mod api;

pub mod domain;

pub mod route;

pub mod service;

pub mod web;

pub mod infra;

pub mod app;
pub mod context;


pub struct AppState {
    pub rbatis: RBatis,
}

