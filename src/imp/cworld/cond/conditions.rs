use crate::imp::mworld::attributes::Attributes;

use super::{condition_item::ConditionItem, condition_items::{ConditionItems, WeightCondItems}};

pub(crate) enum Conditions<T> {
    Seq(ConditionItems<T>),
    Above(ConditionItems<T>),
    One(ConditionItem<T>),
    Random(ConditionItems<T>),
    WeightRandom(WeightCondItems<T>)
}

impl<T> Conditions<T> {
    pub(crate) fn decide(&self, atts: &Attributes) -> Option<&T> {
        match self {
            Self::Seq(c) => c.seq_decide(atts),
            Self::Above(c) => c.seq_decide(atts),
            Self::One(c) => c.decide(atts),
            Self::Random(c) => c.rand_decide(atts),
            Self::WeightRandom(c) => c.rand_decide(atts),
        }
    }

    pub(crate) fn all<'a>(&'a self, atts : &'a Attributes) -> Box<dyn Iterator<Item=&'a T> + 'a>{
        match self {
            Self::Seq(c) => Box::new(c.all_seq(atts)),
            Self::Above(c) => Box::new(c.all(atts)),
            Self::One(c) => Box::new(c.decide(atts).into_iter()),
            Self::Random(c) => Box::new(c.all(atts)),
            Self::WeightRandom(c) => Box::new(c.all(atts)),
        }

    }
}
