use bitboard_proc_macro::{BitboardDebug, BitboardDisplay, bitboard};
use bitboard::{Bitboard, bitboard_table};

// Generate a 5x20 compile-time bitboard
#[bitboard(width = 5, height = 20)]
#[derive(BitboardDisplay, BitboardDebug)]
pub struct Board5x20;

// Optionally generate ray tables (example: vertical and horizontal)
bitboard_table!(RAY_N, ray_n, ray_n_mask, Board5x20, Board5x20::generate_ray_n_table());
bitboard_table!(RAY_S, ray_s, ray_s_mask, Board5x20, Board5x20::generate_ray_s_table());
bitboard_table!(RAY_W, ray_w, ray_w_mask, Board5x20, Board5x20::generate_ray_w_table());
bitboard_table!(RAY_E, ray_e, ray_e_mask, Board5x20, Board5x20::generate_ray_e_table());

// Generate neighbors table
impl Board5x20 {
	pub const NEIGHBORS_8: [Board5x20; 100] = Board5x20::generate_neighbors_8_table();
}

fn main() {
	// Create an empty bitboard
	let mut board = Board5x20::EMPTY;

	// Set some squares
	board.set(2, 3);
	board.set(0, 0);
	board.set(4, 19);

	// Check a square
	if board.get(2, 3) {
		println!("Square (2,3) is set!");
	}

	// Clear a square
	board.reset(0, 0);

	// Print all occupied squares
	println!("Occupied squares (coordinates):");
	for y in 0..Board5x20::HEIGHT {
		for x in 0..Board5x20::WIDTH {
			if board.get(x, y) {
				print!("({},{}) ", x, y);
			}
		}
	}
	println!();

	// Using row and column masks
	let row_mask = Board5x20::row_mask(3);
	let col_mask = Board5x20::col_mask(2);
	println!("Intersection row 3 & col 2:\n{}", row_mask & col_mask);

	// Get neighbors of square (2,3)
	let neighbors = Board5x20::NEIGHBORS_8[Board5x20::index_from_coords(2, 3)];
	println!("Neighbor mask of (2,3):\n{}", neighbors);

	// Example of a sliding ray
	board |= Board5x20::FULL;
	let ray = board.ray_n(Board5x20::index_from_coords(2, 3));
	println!("Sliding ray north from (2,3):\n{}", ray);
}