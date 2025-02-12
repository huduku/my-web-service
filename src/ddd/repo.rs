
use crate::ddd::core::{Identifier, Aggregate, DomainPrimitive, DomainModel};


pub trait Repository {

    type ID : DomainPrimitive<i64> + Identifier;
    type Aggr : DomainModel + Aggregate<Self::ID>;
    
    async fn attach(&self, aggr: Self::Aggr);

    async fn detach(&self, aggr: Self::Aggr);

    async fn find(&self, id: Self::ID) ->  Result<Self::Aggr, String>;

    async fn save(&self, aggr: Self::Aggr) -> Result<(), String>;

    async fn remove(&self, aggr: Self::ID) -> Result<(), String>;

}