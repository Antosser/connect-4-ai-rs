use core::panic;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::boardfunc::{self, place};

static NONE: char = ' ';
static PLAYER: char = 'X';
static AI: char = 'O';

static MAX_DEPTH: i32 = 5;

struct Outcome {
    certainWin: bool,
    certainLoss: bool,
    winChanceNumerator: i32,
    winChanceDenominator: i32,
}
impl Outcome {
    pub fn win() -> Outcome {
        Outcome {
            certainWin: true,
            certainLoss: false,
            winChanceNumerator: 1,
            winChanceDenominator: 1,
        }
    }

    pub fn loss() -> Outcome {
        Outcome {
            certainWin: false,
            certainLoss: true,
            winChanceNumerator: 0,
            winChanceDenominator: 1,
        }
    }

    pub fn unknown() -> Outcome {
        Outcome {
            certainWin: false,
            certainLoss: false,
            winChanceNumerator: 0,
            winChanceDenominator: 0,
        }
    }

    pub fn getChance(self: &Outcome) -> f32 {
        if self.winChanceDenominator == 0 {
            return 0.0;
        }

        self.winChanceNumerator as f32 / self.winChanceDenominator as f32
    }
}

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

    pub fn getValue(self: &TreeNode) -> Outcome {
        let winner = boardfunc::getWinner(&self.board);

        if winner == AI {
            return Outcome::win();
        }
        if winner == PLAYER {
            return Outcome::loss();
        }

        if self.children.is_none() {
            return Outcome::unknown();
        }

        let mut certainWins = 0;
        let mut certainLosses = 0;
        let mut winChanceNumerator = 0;
        let mut winChanceDenominator = 0;
        for child in self.children.as_ref().unwrap().iter() {
            if !child.is_none() {
                let outcome = child.as_ref().unwrap().getValue();

                if outcome.certainWin == true {
                    certainWins += 1;
                }
                if outcome.certainLoss == true {
                    certainLosses += 1;
                }

                winChanceDenominator += outcome.winChanceDenominator;
                winChanceNumerator += outcome.winChanceNumerator;
            }
        }

        if self.nextTurn == PLAYER {
            if certainWins == 10 {
                return Outcome::win();
            }
            if certainLosses > 0 {
                return Outcome::loss();
            }
            return Outcome {
                certainWin: false,
                certainLoss: false,
                winChanceNumerator,
                winChanceDenominator,
            };
        }

        if self.nextTurn == AI {
            if certainWins > 0 {
                return Outcome::win();
            }
            if certainLosses == 10 {
                return Outcome::loss();
            }
            return Outcome {
                certainWin: false,
                certainLoss: false,
                winChanceNumerator,
                winChanceDenominator,
            };
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
            let outcome = TreeNode::new(&afterAIMove, MAX_DEPTH, PLAYER).getValue();

            println!(
                "Move: {} | WinChance: {}/{} | Loss: {} | Win: {}",
                i,
                outcome.winChanceNumerator,
                outcome.winChanceDenominator,
                outcome.certainLoss,
                outcome.certainWin
            );

            let winChance = 'bl: {
                if outcome.certainLoss {
                    break 'bl -1.0;
                }
                if outcome.certainWin {
                    break 'bl 1.0;
                }

                outcome.getChance()
            };

            {
                let mut bestChoicePosition = bestChoicePosition.lock().unwrap();
                let mut bestChoiceProbability = bestChoiceValue.lock().unwrap();

                if winChance > *bestChoiceProbability {
                    *bestChoiceProbability = winChance;
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
