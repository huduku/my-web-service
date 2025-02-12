
use crate::ddd::core::{Identifier, Aggregate};

// #[async_trait::async_trait]
pub trait Repository<ID: Identifier, T : Aggregate<ID>> {

    async fn attach(&self, aggr: T);

    async fn detach(&self, aggr: T);

    async fn find(&self, id: ID) ->  Result<T, String>;

    async fn save(&self, aggr: T) -> Result<(), String>;

    async fn remove(&self, aggr: T) -> Result<(), String>;

}