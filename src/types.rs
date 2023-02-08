#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    Player,
    AI,
    None,
}
impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            match self {
                Player::AI => 'O',
                Player::Player => 'X',
                Player::None => ' ',
            }
        })
    }
}

pub type Board = [[Player; 10]; 10];
