use std::sync::{Arc, Mutex};
use std::thread;

use crate::types::*;

struct Outcome {
    certain_win: bool,
    certain_loss: bool,
    win_chance_numberator: i32,
    win_chance_denominator: i32,
}
impl Outcome {
    pub fn win() -> Outcome {
        Outcome {
            certain_win: true,
            certain_loss: false,
            win_chance_numberator: 1,
            win_chance_denominator: 1,
        }
    }

    pub fn loss() -> Outcome {
        Outcome {
            certain_win: false,
            certain_loss: true,
            win_chance_numberator: 0,
            win_chance_denominator: 1,
        }
    }

    pub fn unknown() -> Outcome {
        Outcome {
            certain_win: false,
            certain_loss: false,
            win_chance_numberator: 0,
            win_chance_denominator: 0,
        }
    }

    pub fn get_chance(self: &Outcome) -> f32 {
        if self.win_chance_denominator == 0 {
            return 0.0;
        }

        self.win_chance_numberator as f32 / self.win_chance_denominator as f32
    }
}

struct TreeNode {
    board: Board,
    children: Option<[Option<Box<TreeNode>>; 10]>,
    next_turn: Player,
}
impl TreeNode {
    pub fn new(board: &Board, depth_left: i32, next_turn: Player) -> TreeNode {
        TreeNode {
            board: board.clone(),
            children: 'bl: {
                if depth_left > 0 {
                    let mut children: [Option<Box<TreeNode>>; 10] =
                        [0; 10].map(|_| -> Option<Box<TreeNode>> { None });
                    for i in 0..10 {
                        let mut new_board = board.clone();
                        if new_board.place(next_turn, i).is_some() {
                            children[i] = Some(Box::new(TreeNode::new(
                                &new_board,
                                depth_left - 1,
                                if next_turn == Player::AI {
                                    Player::Player
                                } else {
                                    Player::AI
                                },
                            )));
                        }
                    }
                    break 'bl Some(children);
                }
                None
            },
            next_turn,
        }
    }

    pub fn get_value(self: &TreeNode) -> Outcome {
        let winner = self.board.get_winner();

        if winner == Player::AI {
            return Outcome::win();
        }
        if winner == Player::Player {
            return Outcome::loss();
        }

        if self.children.is_none() {
            return Outcome::unknown();
        }

        let mut certain_win = 0;
        let mut certain_loss = 0;
        let mut win_chance_numerator = 0;
        let mut win_chance_denominator = 0;

        if self.children.is_none() {
            return Outcome::unknown();
        }
        for child in self.children.as_ref().unwrap().iter() {
            if child.is_some() {
                let outcome = child.as_ref().unwrap().get_value();

                if outcome.certain_win == true {
                    certain_win += 1;
                }
                if outcome.certain_loss == true {
                    certain_loss += 1;
                }

                win_chance_denominator += outcome.win_chance_denominator;
                win_chance_numerator += outcome.win_chance_numberator;
            }
        }

        if self.next_turn == Player::Player {
            if certain_win == 10 {
                return Outcome::win();
            }
            if certain_loss > 0 {
                return Outcome::loss();
            }
            return Outcome {
                certain_win: false,
                certain_loss: false,
                win_chance_numberator: win_chance_numerator,
                win_chance_denominator,
            };
        }

        if self.next_turn == Player::AI {
            if certain_win > 0 {
                return Outcome::win();
            }
            if certain_loss == 10 {
                return Outcome::loss();
            }
            return Outcome {
                certain_win: false,
                certain_loss: false,
                win_chance_numberator: win_chance_numerator,
                win_chance_denominator,
            };
        }

        panic!("wtf");
    }
}

pub fn ai(board: &Board) -> i32 {
    let best_choice_position: Arc<Mutex<f32>> = Arc::new(Mutex::new(0.0));
    let best_choice_probabiliy: Arc<Mutex<f32>> = Arc::new(Mutex::new(-10.0));
    let mut handles: Vec<std::thread::JoinHandle<()>> = vec![];

    println!("AI VALUES");
    for i in 0..10 {
        let best_choice_position = best_choice_position.clone();
        let best_choice_value = best_choice_probabiliy.clone();
        let mut board = board.clone();

        handles.push(thread::spawn(move || {
            if board.place(Player::AI, i).is_none() {
                return;
            }
            let outcome = TreeNode::new(&board, 5, Player::Player).get_value();

            println!(
                "Move: {} | WinChance: {}/{} | Loss: {} | Win: {}",
                i,
                outcome.win_chance_numberator,
                outcome.win_chance_denominator,
                outcome.certain_loss,
                outcome.certain_win
            );

            let win_chance = 'bl: {
                if outcome.certain_loss {
                    break 'bl -1.0;
                }
                if outcome.certain_win {
                    break 'bl 1.0;
                }

                outcome.get_chance()
            };

            {
                let mut best_choice_position = best_choice_position.lock().unwrap();
                let mut best_choice_probability = best_choice_value.lock().unwrap();

                if win_chance > *best_choice_probability {
                    *best_choice_probability = win_chance;
                    *best_choice_position = i as f32;
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    return *best_choice_position.lock().unwrap() as i32;
}
