use std::fmt::Debug;
use crate::ddd::core::{Identifier, Aggregate, DomainPrimitive, DomainModel};


pub trait Repository : Sized {

    type Id: Debug + Clone + Send + Sync;
    type IdGuard: DomainPrimitive<Self::Id> + Identifier;
    type Aggr : DomainModel + Aggregate<Self::IdGuard>;
    
    async fn attach(&self, aggr: Self::Aggr);

    async fn detach(&self, aggr: Self::Aggr);

    async fn find(&self, id: Self::IdGuard) ->  Result<Self::Aggr, String>;

    async fn save(&self, aggr: Self::Aggr) -> Result<(), String>;

    async fn remove(&self, aggr: Self::IdGuard) -> Result<(), String>;

}