use crate::board;
use rand::Rng;

trait Bot {
    fn react(board: &board::GameState) -> char;
}

struct RandomBot {}

impl Bot for RandomBot {
    fn react(board: &board::GameState) -> char {
        let _ = board; // Random bot doesn't check board
        let pos = rand::thread_rng().gen_range(0..6);
        if board.player == board::Player::First {
            return unsafe { char::from_u32_unchecked(('A' as u32) + pos) };
        }
        return unsafe { char::from_u32_unchecked(('a' as u32) + pos) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_bot() {
        let mut game = board::new_game();
        let reaction = RandomBot::react(&game);

        for _ in 0..100 {
            assert!(reaction as usize >= 'A' as usize && reaction as usize <= 'F' as usize);
        }

        game.player = board::Player::Second;
        let reaction = RandomBot::react(&game);
        for _ in 0..100 {
            assert!(reaction as usize >= 'a' as usize && reaction as usize <= 'f' as usize);
        }
    }
}