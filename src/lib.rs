use rbatis::RBatis;

pub mod db;

pub mod domain;

pub mod route;

pub mod dto;

pub mod service;

pub mod controller;

pub struct AppState {
    pub rbatis: RBatis,
}

