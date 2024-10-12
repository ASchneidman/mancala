mod board;

fn main() {
    let mut board = board::new_board();
    board.print();
    board.play('a');
    board.print();
    board.play('b');
    board.print();
    board.play('c');
    board.print();
    board.play('d');
    board.print();
    board.play('e');
    board.print();
    board.play('f');
    board.print();
}
