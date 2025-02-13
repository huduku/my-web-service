use std::marker::PhantomData;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::ddd::dto::{MultipartFile, PageReq};

pub trait Identifier{}
pub trait Identifiable<ID: Identifier> {}

pub trait Entity<ID: Identifier>  : Identifiable<ID> {}

pub trait Aggregate<ID: Identifier>  : Entity<ID> {}



/// CQES: command , query, event, store
pub trait DomainPrimitive<T> : Sized + Debug + Clone + Send + Sync
    where T: Sized + Debug + Clone + Send + Sync
{
    type Error;
    
    fn new(value: Option<T>) -> Result<Self, Self::Error> where Self: Sized;
}

pub trait DomainModel : Sized + Debug + Clone + Send + Sync{
    type CQES : Sized + Debug + Clone + Send + Sync;
    fn new(value: &Self::CQES) -> Result<Self, String>;
}

pub trait MultipartDomainModel : DomainModel
{
    fn new(value: &Self::CQES, multipart_files: Vec<MultipartFile>) -> Result<Self, String>;
}




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

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct IdOper<R, T>
// where
//     R: Send + Sync + Debug + Clone,
//     T: Send + Sync + Identifier + DomainPrimitive<R> {
//     pub id: T,
//     #[serde(skip)] // 避免 `PhantomData` 影响序列化
//     _marker: PhantomData<R>,
// }
// impl<R, T> Identifiable<T> for IdOper<R, T> 
// where R:  Send + Sync + Debug + Clone,
//     T: Send + Sync + Identifier + DomainPrimitive<R>
// {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdOper<T>
where T: Debug + Clone + Send + Sync   {
    pub id: Id<T>
}
impl<T: Debug + Clone + Send + Sync> Identifiable<Id<T>> for IdOper<T> {}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageNo(pub u32);

impl DomainPrimitive<i64> for PageNo {
    type Error = String;
    fn new(value: Option<i64>) -> Result<Self, Self::Error>
    where
        Self: Sized
    {
        match value {
            Some(v) => {
                if v > 0 {
                    return Ok(PageNo(v as u32))
                }
                Err("页码必须大于0".to_string())
            }
            None => Err("页码不能为空".to_string())
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageSize(pub u16);


impl DomainPrimitive<i32> for PageSize {

    type Error = String;
    fn new(value: Option<i32>) -> Result<Self, String> {
        match value {
            Some(v) => {
                if v > 0 {
                    return Ok(PageSize(v as u16))
                }
                Err("每页大小必须大于0".to_string())
            }
            None => Err("每页大小不能为空".to_string())
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
