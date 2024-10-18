#[derive(PartialEq, Clone, Debug)]
pub enum Player {
    First,
    Second,
}

pub struct Pocket {
    pub name: String,
    pub seeds: usize,
    pub player: Player,
}

pub struct Board {
    pub pockets: [Pocket; 14],
}

impl Board {
    pub fn print(&self) {
        println!("    f  e  d  c  b  a");

        fn print_seed(seed: usize) {
            print!("{}", seed);
            if seed < 10 {
                print!(" ");
            }
            print!("|");
        }

        print!("|  |");
        for pocket_index in (8..14).rev() {
            print_seed(self.pockets[pocket_index].seeds);
        }
        print!("  |\n");

        print!("|");
        print_seed(self.pockets[0].seeds);
        print!("-----------------|");
        print_seed(self.pockets[7].seeds);
        print!("\n");
        
        print!("|  |");
        for pocket_index in 1..7 {
            print_seed(self.pockets[pocket_index].seeds);
        }
        print!("  |\n");
        println!("    A  B  C  D  E  F")
    }

    pub fn play(&mut self, position: char) -> Option<usize> {
        // Moves the seeds at position around the board according to the player's turn.
        // Returns the position landed on, if valid
        let mut index;
        if (position as usize >= 'A' as usize) && (position as usize <= 'F' as usize) {
            index = 1 + (position as usize - 'A' as usize);
        } else {
            index = 8 + (position as usize - 'a' as usize);
        }

        let player = self.pockets[index].player.clone();
        if self.pockets[index].seeds == 0 {
            return None;
        }
        let seeds = self.pockets[index].seeds;
        self.pockets[index].seeds = 0;

        let mut seeds_given_out = 0;
        while seeds_given_out < seeds {
            index = (index + 1) % self.pockets.len();
            // skip enemy store
            if self.pockets[index].name == "Store" && self.pockets[index].player != player {
                continue;
            }
            self.pockets[index].seeds += 1;
            seeds_given_out += 1;
        }

        // If we land on our own previously-empty pocket, we capture the enemy adjacent pocket and all go to my store.
        if self.pockets[index].name != "Store" 
            && self.pockets[index].player == player 
            && self.pockets[index].seeds == 1 {
            // Maybe capture! Check if enemy has populated pocket
            let enemy_index = self.pockets.len() - index;
            if self.pockets[enemy_index].seeds != 0 {
                // Capture!
                println!("Capture!");
                self.pockets[if index < 7 { 7 } else { 0 }].seeds += 1 + self.pockets[enemy_index].seeds;
                self.pockets[enemy_index].seeds = 0;
                self.pockets[index].seeds = 0;
            }
        }

        return Some(index);
    }

    pub fn is_game_over(&self) -> bool {
        // Game is over if all of either players' pockets are 
        let mut game_not_over: bool = false;
        for pocket_index in 1..7 {
            // If we see any non-empty pocket, game for this player isn't over
            game_not_over |= self.pockets[pocket_index].seeds != 0;
        }
        if !game_not_over {
            return true;
        }
        game_not_over = false;
        for pocket_index in 8..14 {
            // If we see any non-empty pocket, game for this player isn't over
            game_not_over |= self.pockets[pocket_index].seeds != 0;
        }
        return !game_not_over;
    }
}

pub fn new_board() -> Board {
    let mut pockets: Vec<Pocket> = vec![];
    pockets.push(Pocket {
        name: "Store".into(),
        seeds: 0,
        player: Player::Second,
    });
    for pocket in ["A", "B", "C", "D", "E", "F"] {
        pockets.push(Pocket {
            name: pocket.into(),
            seeds: 4,
            player: Player::First,
        });
    }
    pockets.push(Pocket {
        name: "Store".into(),
        seeds: 0,
        player: Player::First,
    });
    for pocket in ["a", "b", "c", "d", "e", "f"] {
        pockets.push(Pocket {
            name: pocket.into(),
            seeds: 4,
            player: Player::Second,
        });
    }


    return Board {
        pockets: pockets.try_into().unwrap_or_else(|_v| panic!())
    };
}

pub struct GameState {
    pub board: Board,
    pub player: Player,
}

pub fn new_game() -> GameState {
    return GameState {
        board: new_board(),
        player: Player::First,
    }
}

impl GameState {
    pub fn valid_move(&self, position: char) -> bool {
        if self.player == Player::First {
            return (position as usize >= 'A' as usize) && (position as usize <= 'F' as usize);
        }
        return (position as usize >= 'a' as usize) && (position as usize <= 'f' as usize);
    }

    pub fn play(&mut self, position: char) {
        if !self.valid_move(position) {
            return;
        }

        // If we land on a store, stays the same player
        match self.board.play(position) {
            None => return,
            Some(final_pos) => {
                if self.board.pockets[final_pos].name == "Store" {
                    // Player gets another turn
                    return;
                }
                // Next player's turn
                self.player = if self.player == Player::First { Player::Second } else { Player::First };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_correct_player() {
        let mut game = new_game();
        assert_eq!(game.player, Player::First);
        assert_eq!(game.board.pockets[1].seeds, 4);
        game.play('A');
        assert_eq!(game.board.pockets[1].seeds, 0);

        assert_eq!(game.player, Player::Second);
        game.play('a');
        assert_eq!(game.board.pockets[8].seeds, 0);
        assert_eq!(game.player, Player::First);
    }

    #[test]
    fn test_player_gets_another_turn_if_landed_on_store() {
        let mut game = new_game();
        game.play('C');
        assert_eq!(game.player, Player::First);
    }

    #[test]
    fn test_game_over() {
        let mut game = new_game();
        assert!(!game.board.is_game_over());
        for i in 1..7 {
            game.board.pockets[i].seeds = 0;
        }
        assert!(game.board.is_game_over());
    }

    #[test]
    fn test_dont_populate_enemy_store() {
        let mut game = new_game();
        game.board.pockets[6].seeds = 8;
        game.play('F');
        assert_eq!(game.board.pockets[7].seeds, 1);
        // Should wrap around past enemy pocket
        assert_eq!(game.board.pockets[0].seeds, 0);
        assert_eq!(game.board.pockets[1].seeds, 5);
        for i in 8..14 {
            assert_eq!(game.board.pockets[i].seeds, 5);
        }
    }

    #[test]
    fn test_capture_enemy_pocket() {
        let mut game = new_game();
        game.board.pockets[6].seeds = 0;
        game.play('B');
        assert_eq!(game.player, Player::Second);
        assert_eq!(game.board.pockets[6].seeds, 0);
        assert_eq!(game.board.pockets[8].seeds, 0);
        assert_eq!(game.board.pockets[7].seeds, 5);
    }

    #[test]
    fn test_capture_second_enemy_pocket() {
        let mut game = new_game();
        game.play('B');
        game.board.pockets[13].seeds = 0;
        game.play('b');
        assert_eq!(game.player, Player::First);
        assert_eq!(game.board.pockets[13].seeds, 0);
        assert_eq!(game.board.pockets[0].seeds, 5);
        assert_eq!(game.board.pockets[1].seeds, 0);
    }

    #[test]
    fn test_big_move() {
        let mut game = new_game();
        game.board.pockets[0].seeds = 7;
        game.board.pockets[1].seeds = 0;
        game.board.pockets[2].seeds = 0;
        game.board.pockets[3].seeds = 0;
        game.board.pockets[4].seeds = 1;
        game.board.pockets[5].seeds = 9;
        game.board.pockets[6].seeds = 1;
        game.board.pockets[7].seeds = 11;
        game.board.pockets[8].seeds = 1;
        game.board.pockets[9].seeds = 1;
        game.board.pockets[10].seeds = 0;
        game.board.pockets[11].seeds = 0;
        game.board.pockets[12].seeds = 10;
        game.board.pockets[13].seeds = 7;

        // Triggers a capture
        game.play('E');

        assert_eq!(game.board.pockets[7].seeds, 21);
        assert_eq!(game.board.pockets[1].seeds, 0);
        assert_eq!(game.board.pockets[13].seeds, 0);
    }

}