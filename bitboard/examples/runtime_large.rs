use bitboard::{Bitboard, runtime::{RuntimeBitboard, large_bitboard::LargeBitboard}};

fn main() {
	let mut bb = LargeBitboard::empty(19, 19, false);

	println!("Empty bitboard:");
	println!("{}", bb);

	println!("East border:");
	println!("{}", bb.east_border_with_same_shape());

	println!("North border:");
	println!("{}", bb.north_border_with_same_shape());

	println!("South border:");
	println!("{}", bb.south_border_with_same_shape());

	println!("West border:");
	println!("{}", bb.west_border_with_same_shape());

	println!("Full bitboard (debug):");
	println!("{:?}", bb.full_with_same_shape());

	println!("Row mask at row 4:");
	println!("{}", bb.row_mask_with_same_shape(4));

	println!("Column mask at col 13:");
	println!("{}", bb.col_mask_with_same_shape(13));


	let row_bits = bb.full_with_same_shape().extract_row(4);
	bb.insert_row(5, row_bits);
	println!("After inserting row 4 into row 5:");
	println!("{}", bb);
}