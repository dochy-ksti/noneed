use super::battle_status::BattleStatus;



#[derive(Debug, Clone, Copy)]
pub(crate) struct CharaID(pub usize);

pub(crate) struct Chara {
    battle_status: BattleStatus,
}
