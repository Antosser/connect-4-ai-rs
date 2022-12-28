static NONE: char = ' ';
static PLAYER: char = 'X';
static AI: char = 'O';

pub fn printBoard(board: &[[char; 10]; 10]) {
    for y in 0..21 {
        for x in 0..21 {
            let isHorizontal = x % 2 == 0;
            let isVertical = y % 2 == 0;

            if isHorizontal && isVertical {
                print!("+");
            }
            if !isHorizontal && !isVertical {
                print!(" {} ", board[(x - 1) / 2][(y - 1) / 2]);
            }
            if isHorizontal && !isVertical {
                print!("|");
            }
            if !isHorizontal && isVertical {
                print!("---");
            }
        }
        println!();
    }
    println!("  1   2   3   4   5   6   7   8   9   0  ")
}

pub fn place(board: &[[char; 10]; 10], color: char, xPos: usize) -> Option<[[char; 10]; 10]> {
    let mut newBoard = board.clone();

    // println!("PLACE");
    // printBoard(&newBoard);

    for y in (0..10).rev() {
        if board[xPos][y] == NONE {
            newBoard[xPos][y] = color;
            return Some(newBoard);
        }
    }

    None
}

pub fn getWinner(board: &[[char; 10]; 10]) -> char {
    // Horizontal
    for y in 0..10 {
        let mut playerStreak = 0;
        let mut aiStreak = 0;
        for x in 0..10 {
            let cell = board[x][y];

            if cell == PLAYER {
                playerStreak += 1;
                aiStreak = 0;
            }
            if cell == AI {
                aiStreak += 1;
                playerStreak = 0;
            }
            if cell == NONE {
                playerStreak = 0;
                aiStreak = 0;
            }

            if playerStreak == 4 {
                return PLAYER;
            }
            if aiStreak == 4 {
                return AI;
            }
        }
    }

    // Vertical
    for x in 0..10 {
        let mut playerStreak = 0;
        let mut aiStreak = 0;
        for y in 0..10 {
            let cell = board[x][y];

            if cell == PLAYER {
                playerStreak += 1;
                aiStreak = 0;
            }
            if cell == AI {
                aiStreak += 1;
                playerStreak = 0;
            }
            if cell == NONE {
                playerStreak = 0;
                aiStreak = 0;
            }

            if playerStreak == 4 {
                return PLAYER;
            }
            if aiStreak == 4 {
                return AI;
            }
        }
    }

    let edgePieces: [(i32, i32); 19] = [
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

    for edge in edgePieces {
        let mut playerStreak = 0;
        let mut aiStreak = 0;

        let mut pos = edge;
        while (0..10).contains(&pos.0) && (0..10).contains(&pos.1) {
            let cell = board[pos.0 as usize][pos.1 as usize];

            if cell == PLAYER {
                playerStreak += 1;
                aiStreak = 0;
            }
            if cell == AI {
                aiStreak += 1;
                playerStreak = 0;
            }
            if cell == NONE {
                playerStreak = 0;
                aiStreak = 0;
            }

            if playerStreak == 4 {
                return PLAYER;
            }
            if aiStreak == 4 {
                return AI;
            }

            pos.0 += 1;
            pos.1 += 1;
        }
    }

    let otherEdgePieces: [(i32, i32); 19] = [
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

    for edge in otherEdgePieces {
        let mut playerStreak = 0;
        let mut aiStreak = 0;

        let mut pos = edge;
        while (0..10).contains(&pos.0) && (0..10).contains(&pos.1) {
            let cell = board[pos.0 as usize][pos.1 as usize];

            if cell == PLAYER {
                playerStreak += 1;
                aiStreak = 0;
            }
            if cell == AI {
                aiStreak += 1;
                playerStreak = 0;
            }
            if cell == NONE {
                playerStreak = 0;
                aiStreak = 0;
            }

            if playerStreak == 4 {
                return PLAYER;
            }
            if aiStreak == 4 {
                return AI;
            }

            pos.0 -= 1;
            pos.1 += 1;
        }
    }

    NONE
}
