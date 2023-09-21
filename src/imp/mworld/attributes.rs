use bit_vec::BitVec;

use crate::imp::cworld::{charas::chara::CharaID, nodes::node::NodeID};



pub(crate) struct Attributes{
    vec : Vec<AttributeChara>
}

impl Attributes {
    pub(crate) fn new(vec: Vec<AttributeChara>) -> Self { Self { vec } }
    pub(crate) fn attribute(&self, id : CharaID) -> &AttributeChara{
        &self.vec[id.0]
    }
}

pub(crate) struct AttributeChara{
    chara_id : CharaID,
    attributes : BitVec
}

impl AttributeChara {
    pub(crate) fn new(chara_id: CharaID, attributes: BitVec) -> Self { Self { chara_id, attributes } }

    pub(crate) fn has(&self, id : &NodeID) -> bool{
        self.attributes[id.0]
    }
}