use std::fmt::Debug;
use crate::ddd::core::{Identifier, Aggregate, DomainPrimitive, DomainModel};


pub trait Repository : Sized {

    /// 底层数据库原生的类型
    type RawId: Debug + Clone + Send + Sync;
    
    /// Domain Primitive 的 Id， 可以用来做校验防御逻辑
    type Id: DomainPrimitive<Self::RawId> + Identifier;
    
    /// 聚合： 实体类，属于领域模型
    type Aggr : DomainModel + Aggregate<Self::Id>;
    
    async fn attach(&self, aggr: Self::Aggr);

    async fn detach(&self, aggr: Self::Aggr);

    async fn find(&self, id: Self::Id) ->  Result<Self::Aggr, String>;

    async fn save(&self, aggr: Self::Aggr) -> Result<(), String>;

    async fn remove(&self, aggr: Self::Id) -> Result<(), String>;

}