mod ai;
mod tests;
mod types;

use ai::*;
use types::*;

fn main() {
    let mut board: Board = Board::new();
    let mut last_ai_position: Option<i32> = None;

    loop {
        for _ in 0..50 {
            print!("\n");
        }
        println!("PLAYER CHOOSING");

        match last_ai_position {
            Some(pos) => board.print_with_arrow(pos),
            None => board.print(),
        }

        let winner = board.get_winner();
        if winner != Cell::None {
            println!("Winner: {}", winner);
            break;
        }

        loop {
            let mut player_pick = String::new();
            println!("Your pick: ");
            std::io::stdin().read_line(&mut player_pick).unwrap();

            let mut pick_as_int = match player_pick.trim().parse() {
                Ok(x) => x,
                Err(_) => {
                    println!("Error parsing input.");
                    continue;
                }
            };

            pick_as_int -= 1;
            if pick_as_int == -1 {
                pick_as_int = 9;
            }

            if !(0..10).contains(&pick_as_int) {
                println!("Number out of range.");
                continue;
            }

            if board.place(Player::Player, pick_as_int as usize).is_none() {
                println!("No more space in that column");
                continue;
            }

            break;
        }

        println!("AFTER PLAYER CHOOSES");
        board.print();

        let ai_choice = ai(&board);
        println!("\nAi Choice: {}", ai_choice);
        board.place(Player::AI, ai_choice as usize).unwrap();

        last_ai_position = Some(ai_choice);
    }
    let mut freeze = String::new();
    std::io::stdin().read_line(&mut freeze).unwrap();
}
