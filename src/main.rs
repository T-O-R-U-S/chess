mod errors;
mod board;
use board::Board;


fn main() {
	let mut board = Board::new();	

	println!("{}", board);

	board[(3, 1)].unwrap().movement((3, 3), &mut board).unwrap();
	board[(3,6)].unwrap().movement((3,5), &mut board).unwrap();
	

	println!("{}", board)
}
