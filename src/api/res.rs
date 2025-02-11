
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

const SUCCESS: &str = "10000";

const ERROR: &str = "99999";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Res<T> {
    ok: bool,
    code: &'static str,
    data: Option<T>,
    msg: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageRes<T> {
    pub page_no: u64,
    pub page_size: u64,
    pub total: u64,
    pub rows: Option<Vec<T>>,
}


impl<T> Res<T> {
    pub fn is_ok(&self) -> bool {
        self.ok
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    pub fn of(data: T) -> Self {
        Self {
            ok: true,
            code: SUCCESS,
            data: Some(data),
            msg: "".to_string(),
        }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            ok: false,
            code: ERROR,
            data: None,
            msg: msg.into(),
        }
    }
}

impl Res<()> {
    pub fn ok() -> Self {
        Self {
            ok: true,
            code: SUCCESS,
            data: None,
            msg: "".to_string(),
        }
    }
}


impl<T,E> From<Result<T, E>> for Res<T> 
    where E: Display
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(data) => Res::of(data),
            Err(e) => Res::err(e.to_string())
        }
    }
}

impl<T> From<Option<T>> for Res<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(data) => Res::of(data),
            None => Res::err("查询结果为空")
        }
    }
}

// impl<T> From<Page<T>> for PageRes<T>
//     where T: Send + Sync
// {
//     fn from(value: Page<T>) -> Self {
//         PageRes {
//             page_no: value.page_no,
//             page_size: value.page_size,
//             total: value.total,
//             rows: Some(value.records),
//         }
//     }
// }


impl<T> From<PageRes<T>> for Res<PageRes<T>>
{
    fn from(value: PageRes<T>) -> Self {
        Res::of(value)
    }
}

impl<T> Display for Res<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

