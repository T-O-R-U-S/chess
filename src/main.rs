mod pieces;
mod errors;

use pieces::*;

fn main() {
	let board = vec![
		vec![],
		Pawn::pawn_rank(1)
	];

	println!("{:#?}", board)
}
