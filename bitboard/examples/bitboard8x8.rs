use bitboard_proc_macro::{BitboardDebug, BitboardDisplay, bitboard};
use bitboard::{Bitboard, bitboard_table};

// Generate an 8x8 compile-time bitboard
#[bitboard(width = 8, height = 8)]
#[derive(BitboardDisplay, BitboardDebug)]
pub struct Board8x8;

bitboard_table!(RAY_N, ray_n, ray_n_mask, Board8x8, Board8x8::generate_ray_n_table());
bitboard_table!(RAY_S, ray_s, ray_s_mask, Board8x8, Board8x8::generate_ray_s_table());
bitboard_table!(RAY_W, ray_w, ray_w_mask, Board8x8, Board8x8::generate_ray_w_table());
bitboard_table!(RAY_E, ray_e, ray_e_mask, Board8x8, Board8x8::generate_ray_e_table());
bitboard_table!(RAY_NE, ray_ne, ray_ne_mask, Board8x8, Board8x8::generate_ray_ne_table());
bitboard_table!(RAY_NW, ray_nw, ray_nw_mask, Board8x8, Board8x8::generate_ray_nw_table());
bitboard_table!(RAY_SE, ray_se, ray_se_mask, Board8x8, Board8x8::generate_ray_se_table());
bitboard_table!(RAY_SW, ray_sw, ray_sw_mask, Board8x8, Board8x8::generate_ray_sw_table());



bitboard_table!(DIAG_INC, diag_inc, diag_inc_mask, Board8x8, Board8x8::generate_diag_inc_table());
bitboard_table!(DIAG_DEC, diag_dec, diag_dec_mask, Board8x8, Board8x8::generate_diag_dec_table());
bitboard_table!(NEIGHBORS_ORTHO, neighbors_ortho, neighbors_ortho_mask, Board8x8, Board8x8::generate_neighbors_ortho_table());
impl Board8x8 {
	pub const NEIGHBORS_8: [Board8x8; 64] = Board8x8::generate_neighbors_8_table(); 
}
fn main() {
	// Create an empty bitboard
	let mut board = Board8x8::EMPTY;

	// Set some squares
	board.set(3, 4);
	board.set(0, 0);
	board.set(7, 7);

	// Check a square
	if board.get(3, 4) {
		println!("Square (3,4) is set!");
	}

	// Clear a square
	board.reset(0, 0);

	// Print all occupied squares
	println!("Occupied squares (coordinates):");
	for y in 0..Board8x8::HEIGHT {
		for x in 0..Board8x8::WIDTH {
			if board.get(x, y) {
				print!("({},{}) ", x, y);
			}
		}
	}
	println!();

	// Using row and column masks
	let row_mask = Board8x8::row_mask(4); // 5th row
	let col_mask = Board8x8::col_mask(3); // 4th column
	println!("Intersection row 4 & col 3: {}", row_mask & col_mask);

	// Get orthogonal/diagonal neighbors of square (3,4)
	let neighbors = Board8x8::NEIGHBORS_8[Board8x8::index_from_coords(3, 4)];
	println!("Neighbor mask of (3,4):\n {:?}\n{}", neighbors, neighbors);

	// Get orthogonal/diagonal neighbors of square (3,4)
	let neighbors = board.neighbors_ortho(Board8x8::index_from_coords(2, 4));
	println!("Neighbor ortho of (2,4):\n {:?}\n{}", neighbors, neighbors);

	// Example of a sliding ray
	board |= Board8x8::FULL;
	let ray = board.ray_ne(Board8x8::index_from_coords(3, 4));
	println!("Sliding ray (3,4):\n{}", ray);
}