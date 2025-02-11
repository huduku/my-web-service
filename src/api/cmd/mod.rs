use serde::{Deserialize, Serialize};
use crate::ddd::core::Identifier;

pub mod student_cmd;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdCommand<T> where T: Clone +  Identifier {
    pub id: Option<T>
}