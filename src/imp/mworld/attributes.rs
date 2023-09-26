use bit_vec::BitVec;

use crate::imp::cworld::{charas::chara::CharaID, nodes::node::NodeID};

pub(crate) struct CharaAttributes {
    vec: Vec<Attributes>,
}

impl CharaAttributes {
    pub(crate) fn new(vec: Vec<Attributes>) -> Self {
        Self { vec }
    }
    pub(crate) fn attribute(&self, id: CharaID) -> &Attributes {
        &self.vec[id.0]
    }
}

pub(crate) struct CharaAttributeItem {
    chara_id: CharaID,
    attributes: Attributes,
}

pub(crate) struct Attributes {
    attributes: BitVec, //Probably it will be Vec<u8>...?
}

impl Attributes {
    pub(crate) fn has(&self, id: &NodeID) -> bool {
        self.attributes[id.0]
    }
}
