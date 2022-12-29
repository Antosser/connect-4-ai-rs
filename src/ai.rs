use core::panic;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::boardfunc::{self, place};

static NONE: char = ' ';
static PLAYER: char = 'X';
static AI: char = 'O';

static MAX_DEPTH: i32 = 5;

struct TreeNode {
    board: [[char; 10]; 10],
    children: Option<[Option<Box<TreeNode>>; 10]>,
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
                        let newBoard = boardfunc::place(board, nextTurn, i);

                        if !newBoard.is_none() {
                            children[i] = Some(Box::new(TreeNode::new(
                                &newBoard.unwrap(),
                                depthLeft - 1,
                                if nextTurn == AI { PLAYER } else { AI },
                            )));
                        }
                    }
                    break 'bl Some(children);
                }
                None
            },
            nextTurn,
        }
    }

    pub fn getValue(self: &TreeNode) -> f32 {
        let winner = boardfunc::getWinner(&self.board);

        if winner == AI {
            return 1.0;
        }
        if winner == PLAYER {
            return -1.0;
        }

        if self.children.is_none() {
            return 0.0;
        }

        let mut sum: f32 = 0.0;
        let mut maxProbability: f32 = -1.0;
        let mut minProbability: f32 = 1.0;
        for child in self.children.as_ref().unwrap().iter() {
            if !child.is_none() {
                let value = child.as_ref().unwrap().getValue();
                sum += value;

                if value < minProbability {
                    minProbability = value;
                }
                if value > maxProbability {
                    maxProbability = value;
                }
            }
        }
        let average = sum / 10.0;

        if self.nextTurn == PLAYER {
            if minProbability == -1.0 {
                return -1.0;
            }
            return average;
        }

        if self.nextTurn == AI {
            if maxProbability == 1.0 {
                return 1.0;
            }
            return average;
        }

        panic!("wtf");
    }
}

pub fn ai(board: &[[char; 10]; 10]) -> i32 {
    // let mut bestChoicePosition = 0;
    // let mut bestChoiceValue = -10;

    let bestChoicePosition: Arc<Mutex<f32>> = Arc::new(Mutex::new(0.0));
    let bestChoiceProbability: Arc<Mutex<f32>> = Arc::new(Mutex::new(-10.0));
    let mut handles: Vec<std::thread::JoinHandle<()>> = vec![];

    println!("AI VALUES");
    for i in 0..10 {
        let bestChoicePosition = bestChoicePosition.clone();
        let bestChoiceValue = bestChoiceProbability.clone();
        let board = board.clone();

        handles.push(thread::spawn(move || {
            let afterAIMove = place(&board, AI, i).unwrap();
            // println!("AFTER AI MOVE {}", i);
            // boardfunc::printBoard(&afterAIMove);
            let value = TreeNode::new(&afterAIMove, MAX_DEPTH, PLAYER).getValue();

            println!("{}: {}", i, value);

            {
                let mut bestChoicePosition = bestChoicePosition.lock().unwrap();
                let mut bestChoiceProbability = bestChoiceValue.lock().unwrap();

                if value > *bestChoiceProbability {
                    *bestChoiceProbability = value;
                    *bestChoicePosition = i as f32;
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    return *bestChoicePosition.lock().unwrap() as i32;
}
