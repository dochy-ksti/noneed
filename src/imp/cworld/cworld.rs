use self::{charas::Charas, nodes::nodes::Nodes, };




pub(crate) struct CWorld{
    charas : Charas,
    nodes : Nodes,
}

impl CWorld {
    pub(crate) fn new(charas: Charas, nodes: Nodes) -> Self { Self { charas, nodes } }
    pub(crate) fn charas(&self) -> &Charas{ &self.charas }
    pub(crate) fn nodes(&self) -> &Nodes { &self.nodes }
}