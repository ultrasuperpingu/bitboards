use bitboard_proc_macro::{BitboardDebug, BitboardDisplay, bitboard};

// Generate a 12x11 compile-time bitboard
#[bitboard(width = 12, height = 11)]
#[derive(BitboardDisplay, BitboardDebug)]
pub struct Board12x11;



fn main() {
	// Create an empty bitboard
	let mut board = Board12x11::EMPTY;

	// Set some squares
	board.set(5, 5);
	board.set(0, 0);
	board.set(11, 10);

	// Check a square
	if board.get(5, 5) {
		println!("Square (5,5) is set!");
	}

	// Clear a square
	board.reset(0, 0);

	// Print all occupied squares
	println!("Occupied squares (coordinates):");
	for y in 0..Board12x11::HEIGHT {
		for x in 0..Board12x11::WIDTH {
			if board.get(x, y) {
				print!("({},{}) ", x, y);
			}
		}
	}
	println!();

	// Using row and column masks
	let row_mask = Board12x11::row_mask(5);
	let col_mask = Board12x11::col_mask(6);
	println!("Intersection row 5 & col 6: {}", row_mask & col_mask);

}