use crate::Bitboard;


pub mod small_bitboard;
pub mod medium_bitboard;
pub mod large_bitboard;

#[derive(Clone, Debug, )]
pub struct BitboardShape {
	pub w: u8,
	pub h: u8,
	pub col_major: bool,
}
impl BitboardShape {
	pub fn new(w: u8, h: u8, col_major:bool) -> Self {
		Self { w, h, col_major }
	}
}
pub trait RuntimeBitboard : Bitboard {
	fn new(w: u8, h: u8, col_major: bool, bits: Self::Storage) -> Self;

	fn full(w: u8, h: u8, col_major: bool) -> Self;
	fn empty(w: u8, h: u8, col_major: bool) -> Self;

	fn borders(w: u8, h: u8, col_major: bool) -> Self;
	fn west_border(w: u8, h: u8, col_major: bool) -> Self;
	fn east_border(w: u8, h: u8, col_major: bool) -> Self;
	fn south_border(w: u8, h: u8, col_major: bool) -> Self;
	fn north_border(w: u8, h: u8, col_major: bool) -> Self;

	fn row_mask(w: u8, h: u8, col_major: bool, y: u8) -> Self;
	fn col_mask(w: u8, h: u8, col_major: bool, x: u8) -> Self;

	/// Converts `(x, y)` coordinates into a linear bit index.
	fn index_from_coords(w: u8, h: u8, col_major: bool, x: u8, y: u8) -> usize;
	/// Get the (x, y) coords from bit index 
	fn coords_from_index(w: u8, h: u8, col_major: bool, i: usize) -> (u8, u8);
	/// Check (x, y) is inside the bitboard
	fn is_in_bounds(w: u8, h: u8, x: u8, y: u8) -> bool;
	/// Check index is inside the bitboard
	fn is_index_in_bounds(w: u8, h: u8, i: usize) -> bool;

	fn full_with_same_shape(&self) -> Self;
	fn empty_with_same_shape(&self) -> Self;
	
	fn west_border_with_same_shape(&self) -> Self;
	fn east_border_with_same_shape(&self) -> Self;
	fn north_border_with_same_shape(&self) -> Self;
	fn south_border_with_same_shape(&self) -> Self;

	fn row_mask_with_same_shape(&self, y: u8) -> Self;
	fn col_mask_with_same_shape(&self, x: u8) -> Self;

	/// Converts `(x, y)` coordinates into a linear bit index.
	fn index_from_coords_with_same_shape(&self, x: u8, y: u8) -> usize;
	/// Converts a linear bit index into `(x, y)` coordinates.
	fn coords_from_index_with_same_shape(&self, i: usize) -> (u8, u8);
	/// Check (x, y) is inside the bitboard
	fn is_in_bounds_with_same_shape(&self, x: u8, y: u8) -> bool;
	/// Check index is inside the bitboard
	fn is_index_in_bounds_with_same_shape(&self, i: usize) -> bool;

}