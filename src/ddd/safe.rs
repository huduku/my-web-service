use std::fmt::Debug;
use crate::ddd::core::{DomainModel, Id, IdOper, Identifiable, PageNo, PageQuery, PageSize};
use crate::ddd::dto::{MultipartFile, PageReq};

pub trait Safes : Sized + Debug + Clone + Send + Sync {}

impl Safes for u16 {}
impl Safes for u32 {}
impl Safes for i32 {}
impl Safes for i64 {}

impl Safes for String {}

impl<T> Safes for Id<T> where T: Safes {}
impl<T: Safes> Safes for IdOper<T> {}
impl<T: Safes> Identifiable<Id<T>> for IdOper<T> {}
impl Safes for PageNo {}

impl Safes for PageSize {}

impl Safes for MultipartFile {}

impl<DM: DomainModel> Safes for PageQuery<DM> {}

impl<T: Safes> Safes for PageReq<T> {}