
use serde::{Deserialize, Serialize};
use crate::ddd::dto::{MultipartFile, PageReq};

/// CQES: command , query, event, store
pub trait DomainPrimitive<T> : Clone + Send + Sync
    where T: Clone + Send + Sync
{
    type Error;
    
    fn new(value: Option<T>) -> Result<Self, Self::Error> where Self: Sized;
}

pub trait DomainModel : Sized + Clone + Send + Sync{
    type CQES : Clone + Send + Sync;
    fn new(value: &Self::CQES) -> Result<Self, String>;
}

pub trait MultipartDomainModel : DomainModel
{
    fn new(value: &Self::CQES, multipart_files: Vec<MultipartFile>) -> Result<Self, String>;
}

pub trait Identifier{}
pub trait Identifiable<ID: Identifier> {}

pub trait Entity<ID: Identifier>  : Identifiable<ID> {}

pub trait Aggregate<ID: Identifier>  : Entity<ID> {}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Id<T>(pub T);

impl<T> Identifier for Id<T> {}

impl DomainPrimitive<i64> for Id<i64> {
    type Error = String;
    fn new(value: Option<i64>) -> Result<Id<i64>, String> {
        match value {
            Some(v) =>  {
                if v <= 0 {
                    return Err("id 参数非法".to_string());
                }
                Ok(Id(v))
            },
            None => Err("id 不能为空".to_string())
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdOper<T> where T: Send + Sync + Identifier {
    pub id: Id<T>
}

impl Identifier for i64 {}
unsafe impl   Send  for IdOper<i64> {}
unsafe impl   Sync  for IdOper<i64>  {}

impl<T: Send + Sync + Identifier> Identifiable<T> for IdOper<T> {}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageNo(pub u32);

impl DomainPrimitive<u32> for PageNo {
    type Error = String;
    fn new(value: Option<u32>) -> Result<Self, Self::Error>
    where
        Self: Sized
    {
        match value {
            Some(v) => Ok(PageNo(v)),
            None => Ok(PageNo(1)),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageSize(pub u16);


impl DomainPrimitive<u16> for PageSize {

    type Error = String;
    fn new(value: Option<u16>) -> Result<Self, String> {
        match value {
            Some(v) => Ok(PageSize(v)),
            None => Ok(PageSize(10)),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageQuery<DM: DomainModel + Send + Sync + Clone> {
    pub page_no: PageNo,
    pub page_size: PageSize,
    pub query : Option<DM>
}

unsafe impl<DM> Send for PageQuery<DM> where DM: DomainModel + Send + Sync + Clone  {}
unsafe impl<DM> Sync for PageQuery<DM> where DM: DomainModel + Send + Sync + Clone  {}

impl<DM: DomainModel> DomainModel for PageQuery<DM> where DM: DomainModel + Send + Sync + Clone {
    type CQES = PageReq<DM::CQES>;

    fn new(value: &PageReq<DM::CQES>) -> Result<Self, String> {
        PageQuery::<DM>::try_from(value.to_owned())
    }
}

impl<T, DM> TryFrom<PageReq<T>> for PageQuery<DM> 
    where T: Clone,
    DM: DomainModel<CQES=T> + Clone + Send + Sync
{
    type Error = String;
    fn try_from(value: PageReq<T>) -> Result<Self, Self::Error> {
        let page_no = PageNo::new(value.page_no)?;
        let page_size = PageSize::new(value.page_size)?;
        match &value.req {
            None => Ok(PageQuery {
                page_no,
                page_size,
                query: None
            }),
            Some(cqes) => {
                match DM::new(cqes) {
                    Ok(dm) => Ok(PageQuery {
                        page_no,
                        page_size,
                        query: Some(dm),
                    }),
                    Err(e) => Err(e),
                }
            }
        }
    }
}


impl<DM: DomainModel, T: From<DM> + Clone> From<PageQuery<DM>> for PageReq<T> {
    fn from(value: PageQuery<DM>) -> Self {
        match value.query {
            Some(q) => Self {
                page_no: Some(value.page_no.0),
                page_size: Some(value.page_size.0),
                req: Some(q.into())
            },
            None => Self {
                page_no: Some(value.page_no.0),
                page_size: Some(value.page_size.0),
                req: None
            }
        }
    }
}