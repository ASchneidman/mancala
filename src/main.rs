use std::io::stdin;

use bot::Bot;

mod board;
mod bot;

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

fn select_player_type(player: board::Player) -> Option<impl bot::Bot> {
    println!("Select player type for player {:#?}. Hit <Enter> for a real player. Enter the name of the bot + <Enter> for a bot.", player);
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input).ok() {
            None => {
                println!("Select player type for player {:#?}. Hit <Enter> for a real player. Enter the name of the bot + <Enter> for a bot.", player);
            },
            Some(_) => {
                if input == "\n" {
                    return None;
                }
                if input == "RandomBot\n" {
                    return Some(bot::RandomBot {});
                }
            }
        }
    }
}

fn main() {
    let player_one = select_player_type(board::Player::First);
    let player_two = select_player_type(board::Player::Second);
    let mut game = board::new_game();

    while game.board.is_game_over() == None {
        println!("Player {:#?}", game.player);
        game.board.print();
        let participant = match game.player {
            board::Player::First => &player_one,
            board::Player::Second => &player_two,
        };
        match participant {
            None => {
                // Real player
                match read_input() {
                    None => continue,
                    Some(pos) => {
                        game.play(pos);
                    }
                }
            },
            Some(bt) => {
                let mv = bt.react(&game);
                println!("{}", mv);
                game.play(mv);
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
