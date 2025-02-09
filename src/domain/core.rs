use crate::dto::req::MultipartFile;

/// CQES: command , query, event, store
pub trait DomainPrimitive<T>
    where T: Clone + Send + Sync
{
    type Error;
    
    fn new(value: Option<T>) -> Result<Self, Self::Error> where Self: Sized;
}

pub trait DomainModel<DM>
    where DM: Clone + Send + Sync
{
    fn new(value: Self) -> Result<DM, String>;
}

pub trait MultipartDomainModel<DM>: DomainModel<DM>
where DM: Clone + Send + Sync
{
    fn new(value: Self,  multipart_files: Vec<MultipartFile>) -> Result<DM, String>;
}

