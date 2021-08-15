use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
enum PieceError {
	#[error("You made an illegal move.")]
	IllegalMove {
		former_pos: Vec<usize>,
		desired_pos: Vec<usize>
	},
	#[error("You are in check!")]
	InCheck,
	#[error("Your piece is pinned to your king!")]
	DiscoverCheck,
	#[error("There was no piece in your selected tile!")]
	EmptyTile
}