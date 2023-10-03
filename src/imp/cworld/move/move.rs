pub(crate) struct Moves{
    moves : Conditions<Move>
}

pub(crate) struct Percent(usize);

pub(crate) struct MoveCase{
    consume : Percent,
    repeatable : Percent,
}

pub(crate) struct Consume{
    if_succeeded : MoveCase,
    if_failed : MoveCase,
}


pub(crate) struct Move{
    obs : Obstructions,
    consume : Consume,
}