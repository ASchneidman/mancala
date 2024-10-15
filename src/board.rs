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

    pub fn play(&mut self, position: char) -> bool {
        let mut index;
        if (position as usize >= 'A' as usize) && (position as usize <= 'F' as usize) {
            index = 1 + (position as usize - 'A' as usize);
        } else {
            index = 8 + (position as usize - 'a' as usize);
        }

        let player = self.pockets[index].player.clone();
        if self.pockets[index].seeds == 0 {
            return false;
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

        return true;
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
}