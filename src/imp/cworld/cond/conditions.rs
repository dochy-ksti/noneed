use crate::imp::mworld::attributes::AttributeChara;

use super::{condition_item::ConditionItem, condition_items::ConditionItems};


pub(crate) enum Conditions<T> {
    Seq(ConditionItems<T>),
    Above(ConditionItems<T>),
    One(ConditionItem<T>),
    Random(ConditionItems<T>)
}

impl<T> Conditions<T>{
    pub(crate) fn decide(&self, atts : &AttributeChara) -> Option<&T>{
        match self{
            Self::Seq(c) => c.seq_decide(atts),
            Self::Above(c) => c.seq_decide(atts),
            Self::One(c) => c.decide(atts),
            Self::Random(c) => c.rand_decide(atts),
        }
    }
}
