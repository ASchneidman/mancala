use std::io::stdin;

mod board;

fn read_input() -> Option<char> {
    let mut input: String = String::new();
    match stdin().read_line(&mut input).ok() {
        None => return None,
        Some(_) => {
            match input.chars().next() {
                None => return None,
                Some(pos) => {
                    return Some(pos);
                }
            }
        }
    }
}

fn main() {
    let mut game = board::new_game();

    while game.board.is_game_over() == None {
        println!("Player {:#?}", game.player);
        game.board.print();
        match read_input() {
            None => continue,
            Some(pos) => {
                game.play(pos);
            }
        }
    }
    game.board.print();
    match game.board.is_game_over() {
        None => return,
        Some((player, score)) => {
            if score == 24 {
                // Tie!
                println!("Tie!");
                return;
            }
            println!("Winner is {:#?} with a score of {}.", player, score);
        }
    }
}
