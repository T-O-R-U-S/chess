pub mod pieces;
use pieces::*;

use std::{ fmt };

use console::style;

#[derive(Debug, Copy, Clone)]
pub struct Board {
	pub board: [[Option<Piece>; 8]; 8],
	pub in_check: Option<Side>,
	none: Option<Piece>
}

impl Board {
	pub fn pawn_rank(y: isize, side: Side) -> [Option<Piece>; 8] {
		let mut rank = [None; 8];
		for x in 0..8 {
			rank[x] = Some(Piece::Pawn(x as isize, y, side, false))
		}
		rank
	}
	pub fn back_rank(y: isize, side: Side) -> [Option<Piece>; 8] {
		// Returns the default chess setup for the back rank.
		// Must be hard-coded.
		[
			Some(Piece::Rook(0, y, side)),
			Some(Piece::Knight(1, y, side)),
			Some(Piece::Bishop(2, y, side)),
			Some(Piece::King(3, y, side)),
			Some(Piece::Queen(4, y, side)),
			Some(Piece::Bishop(5, y, side)),
			Some(Piece::Knight(6, y, side)),
			Some(Piece::Rook(7, y, side)),
		]
	}
	pub fn empty_row() -> [Option<Piece>; 8] {
		[None; 8]
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
			none: None
		};
		board
	}
}

// Wrapper to implement displaying for Option<Piece>.
pub struct DisplayWrapper(Option<Piece>);

impl fmt::Display for DisplayWrapper {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.0 {
			Some(piece) => write!(f, "{}", style(piece).bold()),
			None => write!(f, "{}", style(".").bg(console::Color::Color256(241)))
		}
	}
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut out = String::new();
		out += "rs| 1 2 3 4 5 6 7 8 |rs\n";
		out += "--|-----------------|--\n";
		for i in 0..8 {
			let y = i as isize;
			out +=
				&format!(
					"{} | {} {} {} {} {} {} {} {} | {}\n",
					i + 1,
					DisplayWrapper(self[(0, y)]),
					DisplayWrapper(self[(1, y)]),
					DisplayWrapper(self[(2, y)]),
					DisplayWrapper(self[(3, y)]),
					DisplayWrapper(self[(4, y)]),
					DisplayWrapper(self[(5, y)]),
					DisplayWrapper(self[(6, y)]),
					DisplayWrapper(self[(7, y)]),
					i+1
				);
		}
		out += "--|-----------------|--\n";
		out += "rs| 1 2 3 4 5 6 7 8 |rs";
		write!(f, "{}", out)
	}
}

impl std::ops::Index<(isize, isize)> for Board {
	type Output = Option<Piece>;
	fn index(&self, index: (isize, isize)) -> &Option<Piece> {
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
	fn index_mut(&mut self, index: (isize, isize)) -> &mut Option<Piece> {
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