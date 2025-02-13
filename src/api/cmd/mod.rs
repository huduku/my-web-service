use serde::{Deserialize, Serialize};
use crate::ddd::core::Safes;

pub mod student_cmd;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdCommand<T> where T: Clone  {
    pub id: Option<T>
}

impl<T: Safes> Safes for  IdCommand<T> {}