use std::ops::Index;

use self::chara::{Chara, CharaID};

pub(crate) mod chara;
pub(crate) mod battle_status;

pub(crate) struct Charas{
    vec : Vec<Chara>
}

impl Charas {
    pub(crate) fn new(vec: Vec<Chara>) -> Self { Self { vec } }
    pub(crate) fn charas(&self) -> &[Chara]{ &self.vec }
    pub(crate) fn chara(&self, id : CharaID) -> &Chara{ self.vec.index(id.0) }
}