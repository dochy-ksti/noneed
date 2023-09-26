use crate::imp::{cworld::nodes::node::NodeID, mworld::attributes::Attributes};

pub(crate) enum Condition {
    Node(NodeID),
    And(Vec<Condition>),
    Or(Vec<Condition>),
}

impl Condition {
    pub(crate) fn matches(&self, atts: &Attributes) -> bool {
        match self {
            Self::Node(id) => atts.has(id),
            Self::And(vec) => vec.iter().all(|cond| cond.matches(atts)),
            Self::Or(vec) => vec.iter().any(|cond| cond.matches(atts)),
        }
    }
}
