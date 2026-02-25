use bitboard::{BitIter, Bitboard, runtime::{RuntimeBitboard, small_bitboard::SmallBitboard}};

fn main() {
	// -------------------------------
	// Example: 7x8 SmallBitboard
	// -------------------------------
	let mut board = SmallBitboard::new(7, 8, false, 0);

	// Set some squares
	board.set(2, 3);
	board.set(0, 0);

	// Check a square
	if board.get(2, 3) {
		println!("Square (2,3) is set!");
	}

	// Print full, empty, and borders
	println!("Full board:\n{}", board.full_with_same_shape());
	println!("Empty board:\n{}", board.empty_with_same_shape());
	println!("North border:\n{}", board.north_border_with_same_shape());
	println!("South border:\n{}", board.south_border_with_same_shape());
	println!("West border:\n{}", board.west_border_with_same_shape());
	println!("East border:\n{}", board.east_border_with_same_shape());

	// Row and column masks
	println!("Row mask 4:\n{}", board.row_mask_with_same_shape(4));
	println!("Column mask 3:\n{}", board.col_mask_with_same_shape(3));

	// Generate sliding moves (e.g., rook-like directions)
	let rook_dirs = [(1,0), (0,1), (-1,0), (0,-1)];
	let sliding_moves = SmallBitboard::generate_sliding_moves(&rook_dirs, board.width(), board.height(), board.col_major());
	println!("Sliding moves from square index 27:\n{}", sliding_moves[27]);

	// Generate jump moves (e.g., knight-like offsets)
	let knight_offsets = [(1,2),(2,1),(2,-1),(1,-2),(-1,-2),(-2,-1),(-2,1),(-1,2)];
	let jump_moves = SmallBitboard::generate_jump_moves(&knight_offsets, board.width(), board.height(), board.col_major());
	println!("Jump moves from square index 27:\n{}", jump_moves[27]);

	// Iterate over all bits set
	println!("Occupied squares:");
	for i in board.iter_bits() {
		let (x, y) = board.coords_from_index_with_same_shape(i as usize);
		println!("({},{})", x, y);
	}
}