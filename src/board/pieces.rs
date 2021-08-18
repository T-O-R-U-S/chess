use crate::board::*;
use crate::errors::PieceError;
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum Side {
	White = -1,
	Black = 1
}

impl std::ops::Mul<isize> for Side {
	type Output = isize;
	fn mul(self, rhs: isize) -> isize{
		match self {
			Side::White => -1*rhs,
			Side::Black => 1*rhs,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Piece {
	// In format of:
	// Piece name (x position, y position, side)
	  Pawn(isize, isize, Side),
	Knight(isize, isize, Side),
	Bishop(isize, isize, Side),
	  Rook(isize, isize, Side),
	 Queen(isize, isize, Side),
	  King(isize, isize, Side),
}

impl Piece {
	fn to_option(&self) -> PieceOption {
		PieceOption::Some(*self)
	}
	pub fn movement(&mut self, new_pos: (isize, isize), board: &mut Board) -> Result<(), PieceError> {
		match *self {
			Piece::  Pawn(x, y, s) => {
				if new_pos.1 == y+(s*1) {
					board[(x, y)] = PieceOption::None;
					board[(x, new_pos.1)] = self.to_option();
					println!("{}", self);
					println!("{}", board[(x, new_pos.1)]);
					println!("{}", board);
					return Ok(())
				}
				return Err(PieceError::IllegalMove)
			},
			_ => Ok(())
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum PieceOption {
	Some(Piece),
	None
}

impl PieceOption {
	pub fn unwrap(self) -> Piece {
		match self {
			PieceOption::Some(val) => val,
			PieceOption::None => panic!("uh oh :(")
		}
	}
	pub fn is_some(&self) -> bool {
		match *self {
			PieceOption::Some(_) => true,
			PieceOption::None => false
		}
	}
}

impl std::fmt::Display for PieceOption {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			PieceOption::Some(val) => val.fmt(f),
			PieceOption::None => write!(f, ".")
		}
	}
}

impl std::fmt::Display for Piece {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			Piece::  Pawn(_, _, _) => write!(f, "P"),
			Piece::Knight(_, _, _) => write!(f, "N"),
			Piece::Bishop(_, _, _) => write!(f, "B"),
			Piece::  Rook(_, _, _) => write!(f, "R"),
			Piece:: Queen(_, _, _) => write!(f, "Q"),
			Piece::  King(_, _, _) => write!(f, "K"),
		}
	}
}

impl std::fmt::Display for Side {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			Side::White => write!(f, "White"),
			Side::Black => write!(f, "Black")
		}
	}
}