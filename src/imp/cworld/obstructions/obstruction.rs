use crate::imp::cworld::charas::chara::CharaID;

use super::battle_data::BattleData;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Difficulty {
    Constant(usize),
    MinMax(usize, usize),
    Normal { mean: usize, std_dev: usize }, //95% of the values are within mean +- std_dev * 2
}

pub(crate) enum Obstruction {
    Battle(CharaID),
    ClearNode,
    Talk(CharaID, Difficulty),
    Info(Difficulty),
}
