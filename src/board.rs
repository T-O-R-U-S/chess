pub mod pieces;
use pieces::*;

use std::{ fmt };

#[derive(Debug, Copy, Clone)]
pub struct Board {
	pub board: [[PieceOption; 8]; 8],
	pub in_check: Option<Side>,
	none: PieceOption
}

impl Board {
	pub fn pawn_rank(y: isize, side: Side) -> [PieceOption; 8] {
		let mut rank = [PieceOption::None; 8];
		for x in 0..8 {
			rank[x] = PieceOption::Some(Piece::Pawn(x as isize, y, side, false))
		}
		rank
	}
	pub fn back_rank(y: isize, side: Side) -> [PieceOption; 8] {
		// Returns the default chess setup for the back rank.
		// Must be hard-coded.
		[
			PieceOption::Some(Piece::Rook(0, y, side)),
			PieceOption::Some(Piece::Knight(1, y, side)),
			PieceOption::Some(Piece::Bishop(2, y, side)),
			PieceOption::Some(Piece::King(3, y, side)),
			PieceOption::Some(Piece::Queen(4, y, side)),
			PieceOption::Some(Piece::Bishop(5, y, side)),
			PieceOption::Some(Piece::Knight(6, y, side)),
			PieceOption::Some(Piece::Rook(7, y, side)),
		]
	}
	pub fn empty_row() -> [PieceOption; 8] {
		[PieceOption::None; 8]
	}
	pub fn new() -> Board {
		let board = Self {
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
			in_check: None,
			none: PieceOption::None
		};
		board
	}
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut out = String::new();
		out += "rs| 1 2 3 4 5 6 7 8 |rs\n";
		out += "--|-----------------|--\n";
		for i in 0..8 {
			out +=
				&format!(
					"{} | {} {} {} {} {} {} {} {} | {}\n",
					i + 1,
					self.board[i][0],
					self.board[i][1],
					self.board[i][2],
					self.board[i][3],
					self.board[i][4],
					self.board[i][5],
					self.board[i][6],
					self.board[i][7],
					i+1
				);
		}
		out += "--|-----------------|--\n";
		out += "rs| 1 2 3 4 5 6 7 8 |rs";
		write!(f, "{}", out)
	}
}

impl std::ops::Index<(isize, isize)> for Board {
	type Output = PieceOption;
	fn index(&self, index: (isize, isize)) -> &PieceOption {
		let usize_index = (index.0 as usize, index.1 as usize);
		if
			// Check if any of the indexes points to an
			// invalid co-ordinate

			// When a negative isize gets cast to a usize,
			// it overflows and becomes the max integer,
			// which is why this check works.
			usize_index.0 > 7 || usize_index.1 > 7
		{
			return &self.none
		}
		// The Y axis (row) gets accessed before the X axis (file)
		&self.board[usize_index.1][usize_index.0]
	}
}

impl std::ops::IndexMut<(isize, isize)> for Board {
	fn index_mut(&mut self, index: (isize, isize)) -> &mut pieces::PieceOption {
		let usize_index = (index.0 as usize, index.1 as usize);
		if 
			// Check if any of the indexes points to an
			// invalid co-ordinate

			// When a negative isize gets cast to a usize,
			// it overflows and becomes the max integer,
			// which is why this check works.
			usize_index.0 > 7 || usize_index.1 > 7
		{
		 return &mut self.none
		}
		// The Y axis (row) gets accessed before the X axis (file)
		&mut self.board[usize_index.1][usize_index.0]
	}
}