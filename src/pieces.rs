// TODO: Make use of this!
#[allow(dead_code)]
enum Side {
	Black = -1,
	White = 1
}

/// The piece trait. This guarantees that every single piece
/// will have a defined set of functions for movement and creation
/// (for when being placed on the board.)
pub trait Piece {
	/// The movement function. Returns a result.
	fn movement(&self) -> Result<Vec<usize>, anyhow::Error>;
	fn new(x: usize, y: usize) -> Self;
}


/// The pawn. This piece can only move forwards, and can only take diagonally (by 1 square).
#[derive(Debug, Clone, Copy)]
pub struct Pawn {
	pub x: usize,
	pub y: usize,
}

impl Piece for Pawn {
	fn movement(&self) -> Result<Vec<usize>, anyhow::Error> {
		Ok(vec![self.x.clone(), self.y.clone()])
	}
	fn new(x: usize, y: usize) -> Pawn {
		Pawn {
			x: x,
			y: y
		}
	}
}

impl Pawn {
	pub fn pawn_rank(y: usize) -> Vec<Pawn> {
		let mut rank = Vec::with_capacity(8);

		for x in 0..8 {
			rank.push(Pawn::new(x, y))
		}

		rank
	}
}

#[derive(Debug, Clone, Copy)]
struct King {
	x: usize,
	y: usize,
}

impl Piece for King {
	fn movement(&self) -> Result<Vec<usize>, anyhow::Error> {
		Ok(vec![self.x.clone(), self.y.clone()])
	}
	fn new(x: usize, y: usize) -> King {
		King {
			x: x,
			y: y
		}
	}
}