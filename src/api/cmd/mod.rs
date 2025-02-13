use serde::{Deserialize, Serialize};

pub mod student_cmd;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdCommand<T> where T: Clone  {
    pub id: Option<T>
}