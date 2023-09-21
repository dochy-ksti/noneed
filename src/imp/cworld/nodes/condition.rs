use crate::imp::mworld::attributes::AttributeChara;

use super::node::NodeID;

pub enum Condition {
    Node(NodeID),
}

pub struct Conditions<T> {
    vec: Vec<ConditionItem<T>>,
}

pub struct ConditionItem<T> {
    cond: Condition,
    item: T,
}

impl<T> Conditions<T> {
    pub fn conds(&self) -> &[ConditionItem<T>] {
        &self.vec
    }

    pub fn decide(&self, atts: &AttributeChara) -> Option<&T> {
        for item in &self.vec {
            if let Some(item) = item.decide(atts) {
                return Some(item);
            }
        }
        None
    }
}

impl<T> ConditionItem<T> {
    pub fn decide(&self, atts: &AttributeChara) -> Option<&T> {
        if self.cond.matches(atts) {
            Some(&self.item)
        } else {
            None
        }
    }
}

impl Condition {
    pub(crate) fn matches(&self, atts: &AttributeChara) -> bool {
        match self {
            Self::Node(id) => atts.has(id),
        }
    }
}
