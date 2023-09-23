use crate::imp::mworld::attributes::AttributeChara;

use super::{condition_item::ConditionItem, condition_items::ConditionItems};


pub(crate) enum Conditions<T> {
    Seq(SeqCond<T>),
    Above(AboveCond<T>),
    One(ConditionItem<T>),
    Random(ConditionItems<T>)
}

pub(crate) struct SeqCond<T>{
    vec : Vec<Conditions<T>>
}

pub(crate) struct AboveCond<T>{
    vec : Vec<Conditions<T>>
}

impl<T> Conditions<T>{
    pub(crate) fn decide(&self, atts : &AttributeChara) -> Option<&T>{
        match self{
            Self::Seq(c) => c.decide(atts),
            Self::Above(c) => c.decide(atts),
            Self::One(c) => c.decide(atts),
            Self::Random(c) =>
        }
    }
}

impl<T> SeqCond<T>{
    pub(crate) fn decide(&self, atts : &AttributeChara) -> Option<&T>{
        let mut result = None;
        for item in &self.vec{
            if let Some(r) = item.decide(atts){
                result = Some(r)
            } else{
                break;
            }
        }
        return result;
    }
}

impl<T> AboveCond<T>{
    pub(crate) fn decide(&self, atts : &AttributeChara) -> Option<&T>{
        for item in &self.vec{
            if let Some(r) = item.decide(atts){
                return Some(r);
            } 
        }
        return None;
    }
}

