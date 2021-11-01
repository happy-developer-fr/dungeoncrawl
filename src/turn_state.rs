#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AwaintingInput,
    PlayerTurn,
    MonsterTurn,
}
