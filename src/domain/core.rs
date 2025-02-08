use crate::dto::req::MultipartFile;

/// CQES: command , query, event, store
pub trait DomainPrimitive<T>
    where T: Clone + Send + Sync
{
    type Error;
    
    fn new(value: Option<T>) -> Result<Self, Self::Error> where Self: Sized;
}

pub trait DomainGuard<DG>
where DG: Clone + Send + Sync
{
    fn new(value: Self) -> Result<DG, String>;
}

pub trait MultipartDomainGuard<DG>: DomainGuard<DG>
where DG: Clone + Send + Sync
{
    fn new(value: Self,  multipart_files: Vec<MultipartFile>) -> Result<DG, String>;
}

pub trait DomainModel<M> : DomainGuard<M> where M: Clone + Send + Sync {}

pub trait DomainEntity<E> : DomainModel<E> where E: Clone + Send + Sync {
    type ID;
}
    
pub trait DomainValueObject<V> : DomainModel<V> where V: Clone + Send + Sync {
    
}

pub trait DomainAggregateRoot<A> : DomainModel<A> where A: Clone + Send + Sync {
    type ID;
}