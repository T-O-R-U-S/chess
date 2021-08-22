use crate::board::*;
use crate::errors::PieceError;
use anyhow::Result;
use console::style;
use console::Color;

#[derive(Debug, Clone, Copy)]
pub enum Side {
	White = -1,
	Black = 1
}

impl Side {
	fn color(&self) -> Color {
		match *self {
			Side::Black => Color::Black,
			Side::White => Color::White
		}
	}
	fn fg(&self) -> Color {
		match *self {
			Side::Black => Color::White,
			Side::White => Color::Black
		}
	}
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
	// bool = has_moved
	  Pawn(isize, isize, Side,  bool),
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
	pub fn set_pos(&self, prev_pos:(isize, isize), new_pos: (isize, isize), board: &mut Board) -> PieceOption {
		board[(prev_pos.0, prev_pos.1)] = PieceOption::None;
		board[(new_pos.0, new_pos.1)] = self.to_option();
		PieceOption::Some(*self)
	}
	pub fn style(&self, notation: &'static str, s: Side) -> console::StyledObject<&'static str> {
		style(notation).bg(s.color()).fg(s.fg())
	}
	pub fn movement(&mut self, new_pos: (isize, isize), board: &mut Board) -> Result<(), PieceError> {
		match *self {
			Piece::  Pawn(x, y, s, has_moved) => {
				if new_pos.0 != x {
					return Err(PieceError::IllegalMove)
				}
				if !has_moved && new_pos.1 == y+(s*2) {
					return Ok(())
				}
				if new_pos.1 == y+(s*1) {
					self.set_pos((x, y), new_pos, board);
					println!("{:?}", board[(x, new_pos.1)]);
					println!("{}", board);
					return Ok(())
				}
				return Err(PieceError::IllegalMove)
			},
			_ => unimplemented!()
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
			PieceOption::None => write!(f, "{}", style(".").bg(Color::Color256(243)).fg(Color::White))
		}
	}
}

impl std::fmt::Display for Piece {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

		match *self {
			Piece::Pawn(_, _, s, _) => write!(f, "{}", self.style("P", s)),
			Piece::Knight(_, _, s)  => write!(f, "{}", self.style("K", s)),
			Piece::Bishop(_, _, s)  => write!(f, "{}", self.style("B", s)),
			Piece::  Rook(_, _, s)  => write!(f, "{}", self.style("R", s)),
			Piece:: Queen(_, _, s)  => write!(f, "{}", self.style("Q", s)),
			Piece::  King(_, _, s)  => write!(f, "{}", self.style("K", s)),
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