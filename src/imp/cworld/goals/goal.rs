use crate::imp::cworld::{nodes::node::NodeID, cond::conditions::Conditions};

pub(crate) struct Goals{
    cond : Conditions<Goal>
}

pub(crate) struct Goal{
    node : NodeID
}