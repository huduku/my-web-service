use serde::{Deserialize, Serialize};
use crate::domain::core::Identifier;

pub mod res;
pub mod req;
pub mod student_cmd;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdCommand<T> where T: Clone +  Identifier {
    pub id: Option<T>
}