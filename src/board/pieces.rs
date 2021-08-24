use crate::board::*;
use crate::errors::PieceError;
use anyhow::Result;
use console::style;
use console::Color;
use anyhow::anyhow;

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
	fn to_option(&self) -> Option<Piece> {
		Some(*self)
	}
	pub fn set_pos(&self, prev_pos:(isize, isize), new_pos: (isize, isize), board: &mut Board) -> Option<Piece> {
		board[prev_pos] = None;
		board[ new_pos] = self.to_option();
		Some(*self)
	}
	pub fn style(notation: &'static str, s: Side) -> console::StyledObject<&'static str> {
		style(notation).bg(s.color()).fg(s.fg())
	}
	pub fn movement(&mut self, new_pos: (isize, isize), board: &mut Board) -> Result<()> {
		match *self {
			Piece::  Pawn(x, y, s, has_moved) => {
				/*
				if new_pos.0 != x || new_pos.0 != x+1 || new_pos.0 == x-1 {
					return Err(anyhow!(PieceError::IllegalMove))
				}
				if new_pos.1 == y+(s*1) {
					self.set_pos((x, y), new_pos, board);
					return Ok(())
				}
				if !has_moved && new_pos.1 == y+(s*2) {
					self.set_pos((x, y), new_pos, board);
					return Ok(())
				} */
				let default = || {

				};

				let x_1 = x+1;
				let y_1 = y+1;

				Err(anyhow!(PieceError::IllegalMove))
			},
			_ => unimplemented!()
		}
	}
}

impl std::fmt::Display for Piece {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

		match *self {
			Piece::Pawn(_, _, s, _) => write!(f, "{}", Self::style("P", s)),
			Piece::Knight(_, _, s,) => write!(f, "{}", Self::style("K", s)),
			Piece::Bishop(_, _, s,) => write!(f, "{}", Self::style("B", s)),
			Piece::  Rook(_, _, s,) => write!(f, "{}", Self::style("R", s)),
			Piece:: Queen(_, _, s,) => write!(f, "{}", Self::style("Q", s)),
			Piece::  King(_, _, s,) => write!(f, "{}", Self::style("K", s)),
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