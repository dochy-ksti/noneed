use crate::imp::{mworld::attributes::AttributeChara, cworld::nodes::node::NodeID};


pub(crate) enum Condition {
    Node(NodeID),
    And(Vec<Condition>),
    Or(Vec<Condition>)
}




impl Condition {
    pub(crate) fn matches(&self, atts: &AttributeChara) -> bool {
        match self {
            Self::Node(id) => atts.has(id),
            Self::And(vec)  => vec.iter().all(|cond| cond.matches(atts)),
            Self::Or(vec) => vec.iter().any(|cond| cond.matches(atts)),
        }
    }
}
