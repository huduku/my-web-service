

// command query event store
pub trait CQES {
    type Command;
    type DomainModel;
    type DataObject;
    
}
