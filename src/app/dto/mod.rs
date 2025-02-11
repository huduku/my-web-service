use serde::{Deserialize, Serialize};
use crate::domain::core::Identifier;

pub mod req;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdCommand<T> where T: Clone +  Identifier {
    pub id: Option<T>
}