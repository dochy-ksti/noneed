pub(crate) struct Moves{
    moves : Conditions<Move>
}

pub(crate) struct ConsumeData{
    percent : usize,
    repeatable : bool,
}

/// In "No" and "IfSucceeded", even if you failed, you can continue your turn, 
/// but You can't do the move repeatedly.
pub(crate) enum Consume{
    No,
    IfSucceeded(usize),
    IfFailed(usize),
    Yes(usize),
}

pub(crate) struct Move{
    obs : Obstructions,
    consume : Consume,
}