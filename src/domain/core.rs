use crate::dto::req::MultipartFile;

/// CQES: command , query, event, store
pub trait DomainPrimitive<T>
    where T: Clone + Send + Sync
{
    type Error;
    
    fn new(value: Option<T>) -> Result<Self, Self::Error> where Self: Sized;
}

// pub trait DomainModel<DM>
//     where DM: Clone + Send + Sync
// {
//     fn of(value: Self) -> Result<DM, String>;
// }


pub trait DomainModel : Sized{
    type CQE : Clone + Send + Sync ;
    fn new(value: &Self::CQE) -> Result<Self, String>;
}

pub trait MultipartDomainModel : DomainModel
{
    fn new(value: &Self::CQE, multipart_files: Vec<MultipartFile>) -> Result<Self, String>;
}

