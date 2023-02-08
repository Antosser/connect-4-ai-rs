static NONE: char = ' ';
static PLAYER: char = 'X';
static AI: char = 'O';

pub fn print_board(board: &[[char; 10]; 10]) {
    for y in 0..21 {
        for x in 0..21 {
            let is_horizontal = x % 2 == 0;
            let is_vertical = y % 2 == 0;

            if is_horizontal && is_vertical {
                print!("+");
            }
            if !is_horizontal && !is_vertical {
                print!(" {} ", board[(x - 1) / 2][(y - 1) / 2]);
            }
            if is_horizontal && !is_vertical {
                print!("|");
            }
            if !is_horizontal && is_vertical {
                print!("---");
            }
        }
        println!();
    }
    println!("  1   2   3   4   5   6   7   8   9   0  ")
}

pub fn place(board: &[[char; 10]; 10], color: char, x_pos: usize) -> Option<[[char; 10]; 10]> {
    let mut new_board = board.clone();

    // println!("PLACE");
    // printBoard(&newBoard);

    for y in (0..10).rev() {
        if board[x_pos][y] == NONE {
            new_board[x_pos][y] = color;
            return Some(new_board);
        }
    }

    None
}

pub fn get_winner(board: &[[char; 10]; 10]) -> char {
    // Horizontal
    for y in 0..10 {
        let mut player_streak = 0;
        let mut ai_streak = 0;
        for x in 0..10 {
            let cell = board[x][y];

            if cell == PLAYER {
                player_streak += 1;
                ai_streak = 0;
            }
            if cell == AI {
                ai_streak += 1;
                player_streak = 0;
            }
            if cell == NONE {
                player_streak = 0;
                ai_streak = 0;
            }

            if player_streak == 4 {
                return PLAYER;
            }
            if ai_streak == 4 {
                return AI;
            }
        }
    }

    // Vertical
    for x in 0..10 {
        let mut player_streak = 0;
        let mut ai_streak = 0;
        for y in 0..10 {
            let cell = board[x][y];

            if cell == PLAYER {
                player_streak += 1;
                ai_streak = 0;
            }
            if cell == AI {
                ai_streak += 1;
                player_streak = 0;
            }
            if cell == NONE {
                player_streak = 0;
                ai_streak = 0;
            }

            if player_streak == 4 {
                return PLAYER;
            }
            if ai_streak == 4 {
                return AI;
            }
        }
    }

    let edge_pieces: [(i32, i32); 19] = [
        (9, 0),
        (8, 0),
        (7, 0),
        (6, 0),
        (5, 0),
        (4, 0),
        (3, 0),
        (2, 0),
        (1, 0),
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (0, 5),
        (0, 6),
        (0, 7),
        (0, 8),
        (0, 9),
    ];

    for edge in edge_pieces {
        let mut player_streak = 0;
        let mut ai_streak = 0;

        let mut pos = edge;
        while (0..10).contains(&pos.0) && (0..10).contains(&pos.1) {
            let cell = board[pos.0 as usize][pos.1 as usize];

            if cell == PLAYER {
                player_streak += 1;
                ai_streak = 0;
            }
            if cell == AI {
                ai_streak += 1;
                player_streak = 0;
            }
            if cell == NONE {
                player_streak = 0;
                ai_streak = 0;
            }

            if player_streak == 4 {
                return PLAYER;
            }
            if ai_streak == 4 {
                return AI;
            }

            pos.0 += 1;
            pos.1 += 1;
        }
    }

    let other_edge_pieces: [(i32, i32); 19] = [
        (9, 0),
        (8, 0),
        (7, 0),
        (6, 0),
        (5, 0),
        (4, 0),
        (3, 0),
        (2, 0),
        (1, 0),
        (0, 0),
        (9, 1),
        (9, 2),
        (9, 3),
        (9, 4),
        (9, 5),
        (9, 6),
        (9, 7),
        (9, 8),
        (9, 9),
    ];

    for edge in other_edge_pieces {
        let mut player_streak = 0;
        let mut ai_streak = 0;

        let mut pos = edge;
        while (0..10).contains(&pos.0) && (0..10).contains(&pos.1) {
            let cell = board[pos.0 as usize][pos.1 as usize];

            if cell == PLAYER {
                player_streak += 1;
                ai_streak = 0;
            }
            if cell == AI {
                ai_streak += 1;
                player_streak = 0;
            }
            if cell == NONE {
                player_streak = 0;
                ai_streak = 0;
            }

            if player_streak == 4 {
                return PLAYER;
            }
            if ai_streak == 4 {
                return AI;
            }

            pos.0 -= 1;
            pos.1 += 1;
        }
    }

    NONE
}
