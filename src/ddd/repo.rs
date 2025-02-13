use std::fmt::Debug;
use crate::ddd::core::{Identifier, Aggregate, DomainPrimitive, DomainModel};


pub trait Repository : Sized {

    type RawId: Debug + Clone + Send + Sync;
    type Id: DomainPrimitive<Self::RawId> + Identifier;
    type Aggr : DomainModel + Aggregate<Self::Id>;
    
    async fn attach(&self, aggr: Self::Aggr);

    async fn detach(&self, aggr: Self::Aggr);

    async fn find(&self, id: Self::Id) ->  Result<Self::Aggr, String>;

    async fn save(&self, aggr: Self::Aggr) -> Result<(), String>;

    async fn remove(&self, aggr: Self::Id) -> Result<(), String>;

}