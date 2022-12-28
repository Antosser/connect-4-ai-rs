#![allow(non_snake_case)]

mod ai;
mod boardfunc;

static NONE: char = ' ';
static PLAYER: char = 'X';
static AI: char = 'O';

fn main() {
    let mut board = [[NONE; 10]; 10];

    loop {
        for _ in 0..50 {
            print!("\n");
        }
        println!("PLAYER CHOOSING");
        boardfunc::printBoard(&board);

        println!("Winner: {}", boardfunc::getWinner(&board));

        let mut playerPick = String::new();
        println!("Your pick: ");
        std::io::stdin().read_line(&mut playerPick).unwrap();

        let mut pickAsInt: i32 = playerPick.trim().parse().unwrap();
        pickAsInt -= 1;
        if pickAsInt == -1 {
            pickAsInt = 9;
        }

        assert!((0..10).contains(&(pickAsInt as usize)));

        board = boardfunc::place(&board, PLAYER, pickAsInt as usize).unwrap();
        println!("AFTER PLAYER CHOOSES");
        boardfunc::printBoard(&board);

        let aiChoice = ai::ai(&board);
        board = boardfunc::place(&board, AI, aiChoice as usize).unwrap();
    }
}
