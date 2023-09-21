use super::route::Route;

#[derive(Debug, Clone, Copy)]
pub(crate) struct NodeID(pub usize);

pub(crate) struct Node {
    id: NodeID,
    routes : Vec<Route>
}
