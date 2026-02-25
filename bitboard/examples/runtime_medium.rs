use bitboard::{
	BitIter, Bitboard,
	runtime::{RuntimeBitboard, medium_bitboard::MediumBitboard},
};

fn main() {
	// -------------------------------
	// Example: 8x16 MediumBitboard (128 bits max)
	// -------------------------------
	let mut board = MediumBitboard::new(8, 16, false, 0);

	// Set some squares
	board.set(5, 5);
	board.set(0, 0);
	board.set(7, 15);

	// Check a square
	if board.get(5, 5) {
		println!("Square (5,5) is set!");
	}

	// Print full, empty, and borders
	println!("Full board:\n{}", board.full_with_same_shape());
	println!("Empty board:\n{}", board.empty_with_same_shape());
	println!("North border:\n{}", board.north_border_with_same_shape());
	println!("South border:\n{}", board.south_border_with_same_shape());
	println!("West border:\n{}", board.west_border_with_same_shape());
	println!("East border:\n{}", board.east_border_with_same_shape());

	// Row and column masks
	println!("Row mask 6:\n{}", board.row_mask_with_same_shape(6));
	println!("Column mask 3:\n{}", board.col_mask_with_same_shape(3));
	for e in board.iter_bits() {
		println!("{:?}", board.coords_from_index_with_same_shape(e as usize))
	}
}