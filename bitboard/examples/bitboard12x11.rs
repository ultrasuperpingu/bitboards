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

	
	board = board.dilated();
	println!("Dilated 1:\n{}", board);
	board = board.dilated();
	println!("Dilated 2:\n{}", board);
	board = board.dilated();
	println!("Dilated 3:\n{}", board);
	board = Board12x11::from_coords(0, 0) | Board12x11::from_coords(11, 0)|Board12x11::from_coords(0, 10)|Board12x11::from_coords(11, 10);
	println!("CORNERS:\n{}", board);
	board = board.dilated();
	println!("Dilated 1:\n{}", board);
	board = board.dilated();
	println!("Dilated 2:\n{}", board);
	board = board.dilated();
	println!("Dilated 3:\n{}", board);

	board = Board12x11::FULL;
	println!("FULL:\n{}", board);
	board = board.eroded();
	println!("Eroded 1:\n{}", board);
	board = board.eroded();
	println!("Eroded 2:\n{}", board);
	board = board.eroded();
	println!("Eroded 3:\n{}", board);
	board = Board12x11::from_coords(0, 0) | Board12x11::from_coords(11, 0)|Board12x11::from_coords(0, 10)|Board12x11::from_coords(11, 10);
	board = board.dilated();
	board = board.dilated();
	println!("Dilated Corners:\n{}", board);
	board = board.eroded();
	println!("Eroded 1:\n{}", board);
	board = board.neighbors_of_any();
	println!("Neighbors:\n{}", board);

}