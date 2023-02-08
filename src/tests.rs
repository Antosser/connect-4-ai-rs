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
            let mut board = [[Player::None; 10]; 10];

            board = place(&board, player, 1).unwrap();

            assert_eq!(get_winner(&board), Player::None);

            board = place(&board, player, 2).unwrap();
            board = place(&board, player, 3).unwrap();
            board = place(&board, player, 4).unwrap();

            assert_eq!(get_winner(&board), player);
        }
    }

    #[test]
    fn vertical_winner() {
        for player in [Player::Player, Player::AI] {
            let mut board = [[Player::None; 10]; 10];

            board = place(&board, player, 1).unwrap();

            assert_eq!(get_winner(&board), Player::None);

            board = place(&board, player, 1).unwrap();
            board = place(&board, player, 1).unwrap();
            board = place(&board, player, 1).unwrap();

            assert_eq!(get_winner(&board), player);
        }
    }

    #[test]
    fn no_winner() {
        assert_eq!(get_winner(&[[Player::None; 10]; 10]), Player::None);
    }

    #[test]
    fn right_up_diagonal_winner() {
        for player in [Player::Player, Player::AI] {
            let mut board = [[Player::None; 10]; 10];

            board[2][8] = player;
            board[3][7] = player;
            board[4][6] = player;
            assert_eq!(get_winner(&board), Player::None);
            board[5][5] = player;
            assert_eq!(get_winner(&board), player);
        }
    }

    #[test]
    fn left_up_diagonal_winner() {
        for player in [Player::Player, Player::AI] {
            let mut board = [[Player::None; 10]; 10];

            board[8][8] = player;
            board[7][7] = player;
            board[6][6] = player;
            assert_eq!(get_winner(&board), Player::None);
            board[5][5] = player;
            assert_eq!(get_winner(&board), player);
        }
    }
}
