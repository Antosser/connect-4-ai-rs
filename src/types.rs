use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Player,
    AI,
    None,
}
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            match self {
                Cell::AI => 'O',
                Cell::Player => 'X',
                Cell::None => ' ',
            }
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    Player,
    AI,
}
impl Player {
    pub fn to_opposite(&self) -> Player {
        match self {
            Player::Player => Player::AI,
            Player::AI => Player::Player,
        }
    }

    pub fn to_cell(&self) -> Cell {
        match self {
            Player::Player => Cell::Player,
            Player::AI => Cell::AI,
        }
    }
}

struct Outcome {
    certain_win: bool,
    possible_loss: bool,
    win_chance_numberator: i32,
    win_chance_denominator: i32,
}
impl Outcome {
    pub fn win() -> Outcome {
        Outcome {
            certain_win: true,
            possible_loss: false,
            win_chance_numberator: 1,
            win_chance_denominator: 1,
        }
    }

    pub fn loss() -> Outcome {
        Outcome {
            certain_win: false,
            possible_loss: true,
            win_chance_numberator: 0,
            win_chance_denominator: 1,
        }
    }

    pub fn unknown() -> Outcome {
        Outcome {
            certain_win: false,
            possible_loss: false,
            win_chance_numberator: 0,
            win_chance_denominator: 1,
        }
    }

    pub fn get_chance(self: &Outcome) -> f32 {
        if self.win_chance_denominator == 0 {
            return 0.0;
        }

        self.win_chance_numberator as f32 / self.win_chance_denominator as f32
    }
}

#[derive(Clone)]
pub struct Board {
    pub data: [[Cell; 10]; 10],
}
impl Board {
    pub fn new() -> Board {
        Board {
            data: [[Cell::None; 10]; 10],
        }
    }

    pub fn print_with_arrow(&self, arrow_pos: i32) {
        for _ in 0..(2 + 4 * arrow_pos) {
            print!(" ");
        }
        println!("v");

        self.print();
    }

    pub fn print(&self) {
        for y in 0..21 {
            for x in 0..21 {
                let is_horizontal = x % 2 == 0;
                let is_vertical = y % 2 == 0;

                if is_horizontal && is_vertical {
                    print!("+");
                }
                if !is_horizontal && !is_vertical {
                    print!(" {} ", self.data[(x - 1) / 2][(y - 1) / 2]);
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

    pub fn place(&mut self, player: Player, x_pos: usize) -> Option<()> {
        for y in (0..10).rev() {
            if self.data[x_pos][y] == Cell::None {
                self.data[x_pos][y] = player.to_cell();

                return Some(());
            }
        }

        None
    }

    pub fn get_winner(&self) -> Cell {
        // Horizontal
        for y in 0..10 {
            let mut player_streak = 0;
            let mut ai_streak = 0;
            for x in 0..10 {
                let cell = self.data[x][y];

                match cell {
                    Cell::Player => {
                        player_streak += 1;
                        ai_streak = 0;
                    }
                    Cell::AI => {
                        ai_streak += 1;
                        player_streak = 0;
                    }
                    Cell::None => {
                        player_streak = 0;
                        ai_streak = 0;
                    }
                }

                if player_streak == 4 {
                    return Cell::Player;
                }
                if ai_streak == 4 {
                    return Cell::AI;
                }
            }
        }

        // Vertical
        for x in 0..10 {
            let mut player_streak = 0;
            let mut ai_streak = 0;
            for y in 0..10 {
                let cell = self.data[x][y];

                match cell {
                    Cell::Player => {
                        player_streak += 1;
                        ai_streak = 0;
                    }
                    Cell::AI => {
                        ai_streak += 1;
                        player_streak = 0;
                    }
                    Cell::None => {
                        player_streak = 0;
                        ai_streak = 0;
                    }
                }

                if player_streak == 4 {
                    return Cell::Player;
                }
                if ai_streak == 4 {
                    return Cell::AI;
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
                let cell = self.data[pos.0 as usize][pos.1 as usize];

                match cell {
                    Cell::Player => {
                        player_streak += 1;
                        ai_streak = 0;
                    }
                    Cell::AI => {
                        ai_streak += 1;
                        player_streak = 0;
                    }
                    Cell::None => {
                        player_streak = 0;
                        ai_streak = 0;
                    }
                }

                if player_streak == 4 {
                    return Cell::Player;
                }
                if ai_streak == 4 {
                    return Cell::AI;
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
                let cell = self.data[pos.0 as usize][pos.1 as usize];

                match cell {
                    Cell::Player => {
                        player_streak += 1;
                        ai_streak = 0;
                    }
                    Cell::AI => {
                        ai_streak += 1;
                        player_streak = 0;
                    }
                    Cell::None => {
                        player_streak = 0;
                        ai_streak = 0;
                    }
                }

                if player_streak == 4 {
                    return Cell::Player;
                }
                if ai_streak == 4 {
                    return Cell::AI;
                }

                pos.0 -= 1;
                pos.1 += 1;
            }
        }

        Cell::None
    }

    fn get_outcome(&self, depth: i32, next_turn: Player, user: Player) -> Outcome {
        let winner = self.get_winner();
        let anti_user = user.to_opposite();

        if winner == user.to_cell() {
            return Outcome::win();
        }
        if winner == anti_user.to_cell() {
            return Outcome::loss();
        }

        if depth <= 0 {
            return Outcome::unknown();
        }

        let mut possible_moves: [Board; 10] = [(); 10].map(|_| self.clone());
        for i in 0..10 {
            possible_moves[i].place(next_turn, i);
        }

        let outcomes =
            possible_moves.map(|board| board.get_outcome(depth - 1, next_turn.to_opposite(), user));

        let mut certain_wins = 0;
        let mut possible_losses = 0;
        let mut win_chance_numerator = 0;
        let mut win_chance_denominator = 0;

        for outcome in &outcomes {
            if outcome.certain_win == true {
                certain_wins += 1;
            }
            if outcome.possible_loss == true {
                possible_losses += 1;
            }

            win_chance_denominator += outcome.win_chance_denominator;
            win_chance_numerator += outcome.win_chance_numberator;
        }

        if next_turn == anti_user {
            if certain_wins == 10 {
                return Outcome::win();
            }
            if possible_losses > 0 {
                return Outcome::loss();
            }
            return Outcome {
                certain_win: false,
                possible_loss: false,
                win_chance_numberator: win_chance_numerator,
                win_chance_denominator,
            };
        }
        if next_turn == user {
            if certain_wins > 0 {
                return Outcome::win();
            }
            if possible_losses == 10 {
                return Outcome::loss();
            }
            return Outcome {
                certain_win: false,
                possible_loss: false,
                win_chance_numberator: win_chance_numerator,
                win_chance_denominator,
            };
        }
        panic!();
    }

    pub fn calculate_best_move(&self, user: Player, depth: i32) -> i32 {
        let best_choice_position: Arc<Mutex<f32>> = Arc::new(Mutex::new(0.0));
        let best_choice_probabiliy: Arc<Mutex<f32>> = Arc::new(Mutex::new(-10.0));
        let mut handles: Vec<std::thread::JoinHandle<()>> = vec![];

        println!("AI VALUES");
        for i in 0..10 {
            let best_choice_position = best_choice_position.clone();
            let best_choice_value = best_choice_probabiliy.clone();
            let mut board = self.clone();

            handles.push(thread::spawn(move || {
                if board.place(user, i).is_none() {
                    return;
                }
                let outcome = board.get_outcome(depth, user.to_opposite(), user);

                println!(
                    "Move: {} | WinChance: {}/{} | Loss: {} | Win: {}",
                    i,
                    outcome.win_chance_numberator,
                    outcome.win_chance_denominator,
                    outcome.possible_loss,
                    outcome.certain_win
                );

                let win_chance = 'bl: {
                    if outcome.possible_loss {
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
}
