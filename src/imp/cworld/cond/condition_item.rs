use crate::imp::mworld::attributes::AttributeChara;

use super::condition::Condition;

pub(crate) struct ConditionItem<T> {
    cond: Condition,
    item: T,
}

impl<T> ConditionItem<T> {
    pub(crate) fn decide(&self, atts: &AttributeChara) -> Option<&T> {
        if self.cond.matches(atts) {
            Some(&self.item)
        } else {
            None
        }
    }
}