use core::panic;
use std::cmp::{max, min};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::boardfunc::{self, place};

static NONE: char = ' ';
static PLAYER: char = 'X';
static AI: char = 'O';

static MAX_DEPTH: i32 = 5;

struct TreeNode {
    board: [[char; 10]; 10],
    children: Option<[Box<TreeNode>; 10]>,
    nextTurn: char,
}
impl TreeNode {
    pub fn new(board: &[[char; 10]; 10], depthLeft: i32, nextTurn: char) -> TreeNode {
        TreeNode {
            board: *board,
            children: 'bl: {
                if depthLeft > 0 {
                    let mut children: [Option<Box<TreeNode>>; 10] =
                        [0; 10].map(|_| -> Option<Box<TreeNode>> { None });
                    for i in 0..10 {
                        children[i] = Some(Box::new(TreeNode::new(
                            &boardfunc::place(board, nextTurn, i)
                                .expect("Couldn't find space to place tile in tree"),
                            depthLeft - 1,
                            if nextTurn == AI { PLAYER } else { AI },
                        )));
                    }
                    break 'bl Some(children.map(|child| child.unwrap()));
                }
                None
            },
            nextTurn,
        }
    }

    pub fn getValue(self: &TreeNode) -> i32 {
        let mut maxValue = -999;
        let mut minValue = 999;

        let winner = boardfunc::getWinner(&self.board);

        if winner == AI {
            return 1;
        }
        if winner == PLAYER {
            return -1;
        }

        if self.children.is_none() {
            return 0;
        }

        for child in self.children.as_ref().unwrap().iter() {
            let value = child.getValue();

            minValue = min(minValue, value);
            maxValue = max(maxValue, value);
        }

        if self.nextTurn == PLAYER {
            return minValue;
        }

        if self.nextTurn == AI {
            return maxValue;
        }

        panic!("wtf");
    }
}

pub fn ai(board: &[[char; 10]; 10]) -> i32 {
    let mut bestChoicePosition = 0;
    let mut bestChoiceValue = -10;

    let shared: Arc<Mutex<(i32, i32)>> = Arc::new(Mutex::new((0, -10)));
    let mut threads: Vec<std::thread::JoinHandle<()>> = vec![];

    println!("AI VALUES");
    for i in 0..10 {
        let afterAIMove = place(board, AI, i).unwrap();
        // println!("AFTER AI MOVE {}", i);
        // boardfunc::printBoard(&afterAIMove);
        let value = TreeNode::new(&afterAIMove, MAX_DEPTH, PLAYER).getValue();

        println!("{}: {}", i, value);

        if value > bestChoiceValue {
            bestChoiceValue = value;
            bestChoicePosition = i;
        }
    }

    return bestChoicePosition as i32;
}
