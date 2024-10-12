mod board;

fn main() {
    let mut board = board::new_board();
    board.print();

    board.first_player_pockets[0].seeds += 1;
}
