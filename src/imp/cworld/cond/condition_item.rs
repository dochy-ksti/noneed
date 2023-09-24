use crate::imp::mworld::attributes::Attributes;

use super::condition::Condition;

pub(crate) struct ConditionItem<T> {
    cond: Condition,
    item: T,
}

impl<T> ConditionItem<T> {
    pub(crate) fn decide(&self, atts: &Attributes) -> Option<&T> {
        if self.cond.matches(atts) {
            Some(&self.item)
        } else {
            None
        }
    }
}
