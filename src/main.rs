mod errors;
mod board;

use board::Board;
use self::board::pieces:: { Piece, PieceOption, Side};


fn main() {
	let mut board = Board::new();	

	println!("{}", board);

	board[(0, 1)].unwrap().movement((2, 2), &mut board).unwrap();
}
