use std::io::stdin;

mod board;

fn read_input(game: &mut board::GameState) -> bool {
    let mut input: String = String::new();
    match stdin().read_line(&mut input).ok() {
        None => return false,
        Some(_) => {
            match input.chars().next() {
                None => return false,
                Some(pos) => {
                    if !game.valid_move(pos) {
                        return false;
                    }
                    game.board.play(pos);
                }
            }
        }
    }
    return true;
}

fn main() {
    let mut game = board::new_game();

    while !game.board.is_game_over() {
        println!("Player {:#?}", game.player);
        game.board.print();
        if read_input(&mut game) {
            game.player = if game.player == board::Player::First { board::Player::Second } else { board::Player::First };
        }
    }
    game.board.print();
}
