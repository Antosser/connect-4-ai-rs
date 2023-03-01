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

                if cell == Cell::Player {
                    player_streak += 1;
                    ai_streak = 0;
                }
                if cell == Cell::AI {
                    ai_streak += 1;
                    player_streak = 0;
                }
                if cell == Cell::None {
                    player_streak = 0;
                    ai_streak = 0;
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

                if cell == Cell::Player {
                    player_streak += 1;
                    ai_streak = 0;
                }
                if cell == Cell::AI {
                    ai_streak += 1;
                    player_streak = 0;
                }
                if cell == Cell::None {
                    player_streak = 0;
                    ai_streak = 0;
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

                if cell == Cell::Player {
                    player_streak += 1;
                    ai_streak = 0;
                }
                if cell == Cell::AI {
                    ai_streak += 1;
                    player_streak = 0;
                }
                if cell == Cell::None {
                    player_streak = 0;
                    ai_streak = 0;
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

                if cell == Cell::Player {
                    player_streak += 1;
                    ai_streak = 0;
                }
                if cell == Cell::AI {
                    ai_streak += 1;
                    player_streak = 0;
                }
                if cell == Cell::None {
                    player_streak = 0;
                    ai_streak = 0;
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
}
