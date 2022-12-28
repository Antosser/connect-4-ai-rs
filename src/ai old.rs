use crate::boardfunc;

static NONE: char = ' ';
static PLAYER: char = 'X';
static AI: char = 'O';
static GOOD: char = 'g';
static BAD: char = 'b';

fn calcWorstCase(board: &[[char; 10]; 10], turnOwner: char, depth: i32, maxDepth: i32) -> char {
    for i in 0..10 {
        let newBoard = boardfunc::place(board, &turnOwner, &i).unwrap();
        let winner = boardfunc::getWinner(&newBoard);

        if winner == PLAYER && turnOwner == PLAYER {
            return BAD;
        }
        if winner == AI && turnOwner == AI {
            return GOOD;
        }

        if depth < maxDepth {
            let nextTurn = {
                if turnOwner == AI {
                    PLAYER
                } else {
                    AI
                }
            };
            let worst = calcWorstCase(&newBoard, nextTurn, depth + 1, maxDepth);

            if nextTurn == PLAYER && worst == BAD {
                return BAD;
            }
            if nextTurn == AI && worst == GOOD {
                return GOOD;
            }
        }
    }

    GOOD
}

pub fn ai(board: &[[char; 10]; 10]) -> Option<i32> {
    for i in 0..10 {
        let newBoard = boardfunc::place(board, &AI, &i).unwrap();

        if boardfunc::getWinner(&newBoard) == AI {
            return Some(i as i32);
        }
    }

    for i in 0..10 {
        let newBoard = boardfunc::place(board, &AI, &i).unwrap();

        if calcWorstCase(&newBoard, PLAYER, 1, 5) == GOOD {
            return Some(i as i32);
        }
    }

    None
}
