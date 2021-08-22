mod errors;
mod board;
use board::Board;


fn main() {
	let mut board = Board::new();	

	println!("{}", board);

	board[(0, 1)].unwrap().movement((0, 2), &mut board).unwrap();
	board[(0,6)].unwrap().movement((0,5), &mut board).unwrap();
}
