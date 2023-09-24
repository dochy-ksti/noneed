use crate::imp::mworld::attributes::AttributeChara;

use super::{condition_item::ConditionItem, rand_from_slice::rand_from_slice};

pub(crate) struct ConditionItems<T> {
    vec: Vec<ConditionItem<T>>,
}

impl<T> ConditionItems<T> {
    /// Returns the item just before the None.
    pub(crate) fn seq_decide(&self, atts: &AttributeChara) -> Option<&T> {
        let mut result = None;
        for item in &self.vec {
            if let Some(r) = item.decide(atts) {
                result = Some(r)
            } else {
                break;
            }
        }
        return result;
    }

    /// Returns the first Some.
    pub(crate) fn above_decide(&self, atts: &AttributeChara) -> Option<&T> {
        for item in &self.vec {
            if let Some(r) = item.decide(atts) {
                return Some(r);
            }
        }
        return None;
    }

    /// Returns Some randomly
    pub(crate) fn rand_decide(&self, atts: &AttributeChara) -> Option<&T> {
        let mut vec = vec![];
        for item in &self.vec {
            if let Some(r) = item.decide(atts) {
                vec.push(r);
            }
        }
        return rand_from_slice(&vec).map(|a| *a);
    }

    pub(crate) fn all<'a>(&'a self, atts : &'a AttributeChara) -> impl Iterator<Item=&'a T> + 'a{
        self.vec.iter().filter_map(|a| a.decide(atts))
    }
}
