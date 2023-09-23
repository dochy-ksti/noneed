use crate::imp::cworld::nodes::condition::Condition;

use super::goal::Goal;

pub struct Goals{
    vec : Vec<GoalChunk>
}
pub struct GoalChunk{
    cond : Condition,
    goal : Goal
}
