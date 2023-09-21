
use super::{obstruction::Obstruction, node::NodeID};


pub(crate) struct Route{
    id : NodeID,
    obstruction : Obstruction
}