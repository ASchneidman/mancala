use crate::board;
use rand::Rng;

pub trait Bot {
    fn react(&self, board: &board::GameState) -> char;
}

pub struct RandomBot {}

impl Bot for RandomBot {
    fn react(&self, board: &board::GameState) -> char {
        let _ = board; // Random bot doesn't check board
        let pos = rand::thread_rng().gen_range(0..6);
        if board.player == board::Player::First {
            return unsafe { char::from_u32_unchecked(('A' as u32) + pos) };
        }
        return unsafe { char::from_u32_unchecked(('a' as u32) + pos) };
    }
}

pub struct CaptureBot {}

impl Bot for CaptureBot {
    fn react(&self, board: &board::GameState) -> char {
        // First, if it can move seeds to the Store, do that
        let pocket_range = if board.player == board::Player::First { (1..7) } else { (8..14) };
        let store_index = if board.player == board::Player::First { 7 } else { 0 };
        for pocket_index in pocket_range {
            if (pocket_index + board.board.pockets[pocket_index].seeds) % board.board.pockets.len() == store_index {
                // Can move seeds to the Store
                return board::Board::index_to_char(pocket_index);
            }
        }
        // Then, if it can capture, do that
        // Then random
        return (RandomBot {}).react(&board);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_bot() {
        let mut game = board::new_game();
        let random_bot = RandomBot {};
        let reaction = random_bot.react(&game);

        for _ in 0..100 {
            assert!(reaction as usize >= 'A' as usize && reaction as usize <= 'F' as usize);
        }

        game.player = board::Player::Second;
        let reaction = random_bot.react(&game);
        for _ in 0..100 {
            assert!(reaction as usize >= 'a' as usize && reaction as usize <= 'f' as usize);
        }
    }

    #[test]
    fn test_capture_bot() {
        let game = board::new_game();
        // On first move, CaptureBot should move from C
        let capture_bot = CaptureBot {};
        let reaction = capture_bot.react(&game);
        assert_eq!(reaction, 'C');
    }
}