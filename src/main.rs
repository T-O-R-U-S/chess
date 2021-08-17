mod errors;
mod pieces;

use pieces::{ Piece, Side, PieceOption };

#[derive(Debug)]
struct Board {
	board: [[PieceOption; 8]; 8],
	in_check: Option<Side>
}

impl Board {
	pub fn pawn_rank(y: usize, side: Side) -> [PieceOption; 8] {
		let mut rank = [PieceOption::None; 8];
		for x in 0..8 {
			rank[x] = PieceOption::Some(Piece::Pawn(x, y, side))
		};
		rank
	}
	pub fn back_rank(y: usize, side: Side) -> [PieceOption; 8] {
		// Returns the default chess setup for the back rank.
		// Must be hard-coded.
		[
			PieceOption::Some(Piece::  Rook(0, y, side)),
			PieceOption::Some(Piece::Knight(1, y, side)),
			PieceOption::Some(Piece::Bishop(2, y, side)),
			PieceOption::Some(Piece::  King(3, y, side)),
			PieceOption::Some(Piece:: Queen(4, y, side)),
			PieceOption::Some(Piece::Bishop(5, y, side)),
			PieceOption::Some(Piece::Knight(6, y, side)),
			PieceOption::Some(Piece::	 Rook(7, y, side)),
		]
	}
	pub fn empty_row() -> [PieceOption; 8] {
		[PieceOption::None; 8]
	}
	pub fn display(&self) {
		println!("rs| 1 2 3 4 5 6 7 8");
		println!("--|----------------");
		for i in 0..8 {
			println!(
				"{} | {} {} {} {} {} {} {} {}",
				i+1,
				self.board[i][0],
				self.board[i][1],
				self.board[i][2],
				self.board[i][3],
				self.board[i][4],
				self.board[i][5],
				self.board[i][6],
				self.board[i][7],
			);
		}
	}
}

fn main() {
	let board = Board {
		board: [
				Board::back_rank(0, Side::Black),
				Board::pawn_rank(1, Side::Black),
				Board::empty_row(),
				Board::empty_row(),
				Board::empty_row(),
				Board::empty_row(),
				Board::pawn_rank(6, Side::White),
				Board::back_rank(7, Side::White),
			],
		in_check: None
	};

	board.display();	
}
