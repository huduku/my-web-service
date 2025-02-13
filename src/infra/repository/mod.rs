
pub mod student;

use crate::api::res::PageRes;
use crate::ddd::core::{DomainModel, PageQuery};
use crate::ddd::dto::PageReq;

use axum::extract::FromRef;
use rbatis::{Error, Page, PageRequest};
use crate::ddd::safe::Safes;

pub struct DbRes<T>(pub Result<T, String>);

impl<T> From<Result<T, Error>> for DbRes<T> {
    fn from(value: Result<T, rbatis::Error>) -> Self {
        DbRes(value.map_err(|_| "数据库异常".to_string()))
    }
}

impl<T: Safes> FromRef<PageReq<T>> for PageRequest {
    fn from_ref(value: &PageReq<T>) -> Self {
        PageRequest::new(
            value.page_no.unwrap_or(1) as u64,
            value.page_size.unwrap_or(10) as u64,
        )
    }
}

impl<DM: DomainModel> FromRef<PageQuery<DM>> for PageRequest {
    fn from_ref(value: &PageQuery<DM>) -> Self {
        PageRequest::new(
            value.page_no.0 as u64,
            value.page_size.0 as u64,
        )
    }
}

impl<DM: DomainModel> From<PageQuery<DM>> for PageRequest {
    fn from(value: PageQuery<DM>) -> Self {
        Self::new(value.page_no.0 as u64, value.page_size.0 as u64)
    }
}

impl<T: Safes, DM: DomainModel<CQES=T> + TryFrom<T>> TryFrom<Page<T>> for PageRes<DM> {
    type Error = String;
    fn try_from(value: Page<T>) -> Result<Self, Self::Error> {
        let records = value.records;
        let rows: Vec<DM> = records.into_iter()
            .map(|r| DM::new(&r))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("{}: {}", "存储数据非法", e))?;
        Ok(Self {
            page_no: value.page_no,
            page_size: value.page_size,
            total: value.total,
            rows: Some(rows)
        })
    }
}