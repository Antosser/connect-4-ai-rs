mod ai;
mod boardfunc;
mod tests;
mod types;

use ai::*;
use boardfunc::*;
use types::*;

fn main() {
    let mut board: Board = [[Player::None; 10]; 10];

    loop {
        for _ in 0..50 {
            print!("\n");
        }
        println!("PLAYER CHOOSING");
        print_board(&board);

        let winner = get_winner(&board);
        println!("Winner: {}", winner);
        if winner != Player::None {
            break;
        }

        let mut player_pick = String::new();
        println!("Your pick: ");
        std::io::stdin().read_line(&mut player_pick).unwrap();

        let mut pick_as_int: i32 = player_pick.trim().parse().unwrap();
        pick_as_int -= 1;
        if pick_as_int == -1 {
            pick_as_int = 9;
        }

        assert!((0..10).contains(&(pick_as_int as usize)));

        board = place(&board, Player::Player, pick_as_int as usize).unwrap();
        println!("AFTER PLAYER CHOOSES");
        boardfunc::print_board(&board);

        let ai_choice = ai(&board);
        board = place(&board, Player::AI, ai_choice as usize).unwrap();
    }
    let mut freeze = String::new();
    std::io::stdin().read_line(&mut freeze).unwrap();
}
