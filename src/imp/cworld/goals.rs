use self::goal::Goal;

use super::nodes::condition::Condition;

pub(crate) mod goal;

pub struct Goals{
    vec : Vec<GoalChunk>
}
pub struct GoalChunk{
    cond : Condition,
    goal : Goal
}
