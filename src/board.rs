#[derive(PartialEq, Clone)]
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
    pub first_player_pockets: [Pocket; 6],
    pub first_player_store: Pocket,
    pub second_player_pockets: [Pocket; 6],
    pub second_player_store: Pocket,
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
        for pocket in self.second_player_pockets.iter().rev() {
            print_seed(pocket.seeds);
        }
        print!("  |\n");

        print!("|");
        print_seed(self.second_player_store.seeds);
        print!("-----------------|");
        print_seed(self.first_player_store.seeds);
        print!("\n");
        
        print!("|  |");
        for pocket in self.first_player_pockets.iter() {
            print_seed(pocket.seeds);
        }
        print!("  |\n");
        println!("    A  B  C  D  E  F")
    }

    pub fn play(&mut self, position: char) -> bool {
        let mut pockets: Vec<&mut Pocket> = vec![];
        pockets.push(&mut self.second_player_store);
        for pocket in self.first_player_pockets.iter_mut() {
            pockets.push(pocket);
        }
        pockets.push(&mut self.first_player_store);
        for pocket in self.second_player_pockets.iter_mut() {
            pockets.push(pocket);
        }

        let mut index;
        if (position as usize >= 'A' as usize) && (position as usize <= 'F' as usize) {
            index = 1 + (position as usize - 'A' as usize);
        } else {
            index = 8 + (position as usize - 'a' as usize);
        }

        let player = pockets[index].player.clone();
        if pockets[index].seeds == 0 {
            return false;
        }
        let seeds = pockets[index].seeds;
        pockets[index].seeds = 0;

        let mut seeds_given_out = 0;
        while seeds_given_out < seeds {
            index = (index + 1) % pockets.len();
            // skip enemy store
            if pockets[index].name == "Store" && pockets[index].player != player {
                continue;
            }
            pockets[index].seeds += 1;
            seeds_given_out += 1;
        }

        return true;
    }
}

pub fn new_board() -> Board {
    let mut first_player_pockets: Vec<Pocket> = vec![];
    let mut second_player_pockets: Vec<Pocket> = vec![];
    for name in ["A", "B", "C", "D", "E", "F"] {
        first_player_pockets.push(Pocket {
            name: name.into(),
            seeds: 4,
            player: Player::First,
        });
    }

    for name in ["a", "b", "c", "d", "e", "f"] {
        second_player_pockets.push(Pocket {
            name: name.into(),
            seeds: 4,
            player: Player::Second,
        });
    }


    return Board {
        first_player_pockets: first_player_pockets.try_into().unwrap_or_else(|_v| panic!()),
        first_player_store: Pocket {
            name: "Store".into(),
            seeds: 0,
            player: Player::First,
        },
        second_player_pockets: second_player_pockets.try_into().unwrap_or_else(|_v| panic!()),
        second_player_store: Pocket {
            name: "Store".into(),
            seeds: 0,
            player: Player::Second,
        },
    };
}
