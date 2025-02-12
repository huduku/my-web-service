use crate::ddd::core::DomainModel;

// command query event store, view, domain model
pub trait CQESV {
    type Command : From<Self::Model>;

    type Query : From<Self::Model> ;
    type Event : From<Self::Model> ;
    type Store : From<Self::Model>;

    type View : From<Self::Model>;

    type Model : 
        DomainModel
        + TryFrom<Self::Command> + TryFrom<Self::Query> +  TryFrom<Self::Event> + TryFrom<Self::Store>;



}
