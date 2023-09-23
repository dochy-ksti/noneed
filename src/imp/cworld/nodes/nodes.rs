use super::node::Node;

pub(crate) struct Nodes{
    vec : Vec<Node>
}

impl Nodes {
    pub(crate) fn new(vec: Vec<Node>) -> Self { Self { vec } }

    pub(crate) fn vec(&self) -> &[Node] {
        self.vec.as_ref()
    }
}