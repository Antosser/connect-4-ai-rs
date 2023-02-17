#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn player_eq() {
        assert!(Player::Player == Player::Player);
        assert!(Player::AI == Player::AI);
        assert!(Player::None == Player::None);
    }

    #[test]
    fn horizontal_winner() {
        for player in [Player::Player, Player::AI] {
            let mut board = Board::new();

            assert_eq!(board.get_winner(), Player::None);
            board.data[4][4] = player;
            assert_eq!(board.get_winner(), Player::None);
            board.data[5][4] = player;
            assert_eq!(board.get_winner(), Player::None);
            board.data[6][4] = player;
            assert_eq!(board.get_winner(), Player::None);
            board.data[7][4] = player;
            assert_eq!(board.get_winner(), player);
        }
    }

    #[test]
    fn vertical_winner() {
        for player in [Player::Player, Player::AI] {
            let mut board = Board::new();

            assert_eq!(board.get_winner(), Player::None);
            board.data[4][4] = player;
            assert_eq!(board.get_winner(), Player::None);
            board.data[4][5] = player;
            assert_eq!(board.get_winner(), Player::None);
            board.data[4][6] = player;
            assert_eq!(board.get_winner(), Player::None);
            board.data[4][7] = player;
            assert_eq!(board.get_winner(), player);
        }
    }

    #[test]
    fn no_winner() {
        assert_eq!(Board::new().get_winner(), Player::None);
    }

    #[test]
    fn right_up_diagonal_winner() {
        for player in [Player::Player, Player::AI] {
            let mut board = Board::new();

            assert_eq!(board.get_winner(), Player::None);
            board.data[2][8] = player;
            assert_eq!(board.get_winner(), Player::None);
            board.data[3][7] = player;
            assert_eq!(board.get_winner(), Player::None);
            board.data[4][6] = player;
            assert_eq!(board.get_winner(), Player::None);
            board.data[5][5] = player;
            assert_eq!(board.get_winner(), player);
        }
    }

    #[test]
    fn left_up_diagonal_winner() {
        for player in [Player::Player, Player::AI] {
            let mut board = Board::new();

            assert_eq!(board.get_winner(), Player::None);
            board.data[2][3] = player;
            assert_eq!(board.get_winner(), Player::None);
            board.data[3][4] = player;
            assert_eq!(board.get_winner(), Player::None);
            board.data[4][5] = player;
            assert_eq!(board.get_winner(), Player::None);
            board.data[5][6] = player;
            assert_eq!(board.get_winner(), player);
        }
    }
}
