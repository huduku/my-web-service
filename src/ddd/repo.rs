
use crate::ddd::core::{Identifier, Aggregate};
pub trait Repository<ID: Identifier, T : Aggregate<ID>> {

    async fn attach(aggr: T);

    async fn detach(aggr: T);

    async fn find(id: ID) ->  Result<T, String>;

    async fn save(aggr: T);

    async fn remove(aggr: T);

}