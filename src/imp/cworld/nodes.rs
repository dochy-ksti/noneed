use self::node::Node;

pub(crate) mod node;
pub(crate) mod obstruction;
pub(crate) mod route;
pub(crate) mod battle_data;
pub(crate) mod condition;

pub(crate) struct Nodes{
    vec : Vec<Node>
}

impl Nodes {
    pub(crate) fn new(vec: Vec<Node>) -> Self { Self { vec } }

    pub(crate) fn vec(&self) -> &[Node] {
        self.vec.as_ref()
    }
}