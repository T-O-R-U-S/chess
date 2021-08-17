#[derive(Debug, Clone, Copy)]
pub enum Side {
	White = 1,
	Black = -1
}

#[derive(Debug, Clone, Copy)]
pub enum Piece {
	// In format of:
	// Piece name (x position, y position, notation)
	Pawn(usize, usize, Side),
	Knight(usize, usize, Side),
	Bishop(usize, usize, Side),
	Rook(usize, usize, Side),
	Queen(usize,usize, Side),
	King(usize, usize, Side),
}

impl Piece {
	pub fn movement(&mut self) {

	}
}

#[derive(Debug, Clone, Copy)]
pub enum PieceOption {
	Some(Piece),
	None
}

impl std::fmt::Display for PieceOption {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			PieceOption::Some(Piece::  Pawn(_, _, _)) => write!(f, "P"),
			PieceOption::Some(Piece::Knight(_, _, _)) => write!(f, "N"),
			PieceOption::Some(Piece::Bishop(_, _, _)) => write!(f, "B"),
			PieceOption::Some(Piece::  Rook(_, _, _)) => write!(f, "R"),
			PieceOption::Some(Piece:: Queen(_, _, _)) => write!(f, "Q"),
			PieceOption::Some(Piece::  King(_, _, _)) => write!(f, "K"),
			PieceOption::None => write!(f, " ")
		}
	}
}