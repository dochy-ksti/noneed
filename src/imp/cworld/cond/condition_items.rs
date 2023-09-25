use crate::imp::mworld::attributes::Attributes;

use super::{
    choose_randomly::{rand_from_slice, rand_with_weight},
    conditions::Conditions,
};

pub(crate) struct ConditionItems<T> {
    vec: Vec<Conditions<T>>,
}

impl<T> ConditionItems<T> {
    /// Returns the Some just before the first None. The last one if there's no None. None if there's no Some.
    pub(crate) fn seq_decide(&self, atts: &Attributes) -> Option<&T> {
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
    pub(crate) fn above_decide(&self, atts: &Attributes) -> Option<&T> {
        for item in &self.vec {
            if let Some(r) = item.decide(atts) {
                return Some(r);
            }
        }
        return None;
    }

    /// Returns Some randomly
    pub(crate) fn rand_decide(&self, atts: &Attributes) -> Option<&T> {
        let mut vec = vec![];
        for item in &self.vec {
            if let Some(r) = item.decide(atts) {
                vec.push(r);
            }
        }
        return rand_from_slice(&vec).map(|a| *a);
    }

    pub(crate) fn all<'a>(&'a self, atts: &'a Attributes) -> impl Iterator<Item = &'a T> {
        self.vec.iter().filter_map(|a| a.decide(atts))
    }

    /// In Seq's model, only the latest one is active.
    pub(crate) fn all_seq(&self, atts: &Attributes) -> impl Iterator<Item = &T> {
        Self::seq_decide(self, atts).into_iter()
    }
}

pub(crate) struct WeightCondItems<T> {
    vec: Vec<WeightCondItem<T>>,
}

pub(crate) struct WeightCondItem<T> {
    weight: usize,
    cond: Conditions<T>,
}

impl<T> WeightCondItems<T> {
    /// Returns Some randomly
    pub(crate) fn rand_decide(&self, atts: &Attributes) -> Option<&T> {
        let mut vec = vec![];
        let mut weight_sum = 0;
        for item in &self.vec {
            if let Some(r) = item.decide(atts) {
                vec.push((item.weight, r));
                weight_sum += item.weight;
            }
        }
        return rand_with_weight(vec.into_iter(), weight_sum);
    }

    pub(crate) fn all<'a>(&'a self, atts: &'a Attributes) -> impl Iterator<Item = &'a T> + 'a {
        self.vec.iter().filter_map(|a| a.decide(atts))
    }
}

impl<T> WeightCondItem<T> {
    pub(crate) fn decide(&self, atts: &Attributes) -> Option<&T> {
        self.cond.decide(atts)
    }
}
