#[cfg(test)]
mod tests {
	use bitboard::Bitboard;
	use bitboard_proc_macro::{BitboardDebug, BitboardDisplay, bitboard};


	#[bitboard(width=7,height=7, col_major=true)]
	#[derive(Default, BitboardDebug, BitboardDisplay, Hash)]
	pub struct Bitboard7x7Col;
	
	fn from_indices(indices: &[u8]) -> Bitboard7x7Col {
		let mut bb = Bitboard7x7Col::empty();
		for &i in indices {
			bb.set_at_index(i as usize);
		}
		bb
	}

	#[test]
	fn test_horizontal_alignment() {
		
		let bb = from_indices(&[0, 7, 14, 21]);
		println!("{bb}");
		assert!(bb.has_n_aligned_horizontal(4), "Horizontal alignment not detected");
		
		let bb = from_indices(&[21, 28, 35, 42]);
		println!("{bb}");
		assert!(bb.has_n_aligned_horizontal(4), "Horizontal alignment not detected");

		let bb = from_indices(&[21+6, 28+6, 35+6, 42+6]);
		println!("{bb}");
		assert!(bb.has_n_aligned_horizontal(4), "Horizontal alignment not detected");
		
		
		let bleed = from_indices(&[35, 42, 49, 0]); 
		assert!(!bleed.has_n_aligned_horizontal(4), "Horizontal bleeding detected!");
	}

	#[test]
	fn test_vertical_alignment() {
		let bb = from_indices(&[0, 1, 2, 3]);
		println!("{bb}");
		assert!(bb.has_n_aligned_vertical(4), "Vertical alignment not detected");

		let bleed = from_indices(&[5, 6, 7, 8]);
		assert!(!bleed.has_n_aligned_vertical(4), "Vertical bleeding detected!");
	}

	#[test]
	fn test_diagonal_inc_alignment() {
		let bb = from_indices(&[0, 8, 16, 24]);
		println!("{}", Bitboard7x7Col::DIAG_INC_OFFSET);
		println!("{}", Bitboard7x7Col::DIAG_DEC_OFFSET);
		println!("{bb}");
		assert!(bb.has_n_aligned_diag_inc(4), "Diagonal INC alignment not detected");
	}

	#[test]
	fn test_diagonal_dec_alignment() {
		let bb = from_indices(&[3, 9, 15, 21]);
		println!("{bb}");
		assert!(bb.has_n_aligned_diag_dec(4), "Diagonal DEC alignment not detected");
		assert!(!bb.has_n_aligned_diag_inc(4), "Diagonal INC alignment incorrectly detected");

		let bb = from_indices(&[6, 12, 18, 24]);
		println!("{bb}");
		assert!(bb.has_n_aligned_diag_dec(4), "Diagonal DEC alignment not detected");
		assert!(!bb.has_n_aligned_diag_inc(4), "Diagonal INC alignment incorrectly detected");
		
	}

	#[test]
	fn test_n_parameter() {
		let bb = from_indices(&[0, 1, 2, 3, 4]);
		assert!(bb.has_n_aligned(5));
		assert!(bb.has_n_aligned(2));
		assert!(!bb.has_n_aligned(6));
	}
}


#[cfg(test)]
mod tests_8x8 {
	use bitboard::Bitboard;
	use bitboard_proc_macro::{BitboardDebug, BitboardDisplay, bitboard};

	#[bitboard(width=8,height=8, col_major=false)]
	#[derive(Default, BitboardDebug, BitboardDisplay, Hash)]
	pub struct Bitboard8x8;
	


	fn from_indices(indices: &[u8]) -> Bitboard8x8 {
		let mut bb = Bitboard8x8::empty();
		for &i in indices {
			bb.set_at_index(i as usize);
		}
		bb
	}

	#[test]
	fn test_8x8_vertical() {
		println!("Bitboard8x8::H_OFFSET: {}", Bitboard8x8::H_OFFSET);
		println!("Bitboard8x8::V_OFFSET: {}", Bitboard8x8::V_OFFSET);
		println!("Bitboard8x8::DIAG_DEC_OFFSET: {}", Bitboard8x8::DIAG_DEC_OFFSET);
		println!("Bitboard8x8::DIAG_INC_OFFSET: {}", Bitboard8x8::DIAG_INC_OFFSET);
		let bb = from_indices(&[0, 8, 16, 24, 32]);
		println!("{bb}");
		assert!(bb.has_n_aligned_vertical(5), "Vertical 5-align not detected");
		
		let bb = from_indices(&[24, 32, 40, 48, 56]);
		println!("{bb}");
		assert!(bb.has_n_aligned(5), "Vertical 5-align not detected");

		let bb = from_indices(&[0+7, 8+7, 16+7, 24+7, 32+7]);
		println!("{bb}");
		assert!(bb.has_n_aligned_vertical(5), "Vertical 5-align not detected");
		
		let bb = from_indices(&[24+7, 32+7, 40+7, 48+7, 56+7]);
		println!("{bb}");
		assert!(bb.has_n_aligned(5), "Vertical 5-align not detected");

		let bleed = from_indices(&[40, 48, 56, 1]); 
		assert!(!bleed.has_n_aligned(4), "Vertical bleeding");
	}

	#[test]
	fn test_8x8_horizontal() {
		// Colonne 0 : 0, 1, 2, 3...
		let bb = from_indices(&[10, 11, 12, 13]);
		assert!(bb.has_n_aligned_horizontal(4), "Horizontal 4-align not detected");

		let bleed = from_indices(&[6, 7, 8, 9]);
		assert!(!bleed.has_n_aligned_horizontal(4), "Bleeding Horizontal");
	}

	#[test]
	fn test_8x8_diagonals() {
		let bb_inc = from_indices(&[0, 9, 18, 27]);
		println!("{}", bb_inc);
		assert!(bb_inc.has_n_aligned_diag_inc(4), "Diagonal INC (/) not detected");
		assert!(!bb_inc.has_n_aligned_diag_dec(4), "Diagonal DEC (\\) incorrectly detected");

		let bb_dec = from_indices(&[7, 14, 21, 28]);
		println!("{}", bb_dec);
		assert!(bb_dec.has_n_aligned_diag_dec(4), "Diagonal DEC (\\) not detected");
		assert!(!bb_dec.has_n_aligned_diag_inc(4), "Diagonal INC (/) incorrectly detected");
	}

	#[test]
	fn test_8x8_edge_cases() {
		let bb = from_indices(&[0, 1, 8, 9]);
		assert!(!bb.has_n_aligned(4), "4-alignment detected on a 2x2 square");

		let full_row = from_indices(&[0, 8, 16, 24, 32, 40, 48, 56]);
		assert!(full_row.has_n_aligned(8));
		assert!(!full_row.has_n_aligned(9));
	}
}


#[cfg(test)]
mod tests_goban {
	use bitboard_proc_macro::{BitboardDebug, BitboardDisplay, bitboard};

	#[bitboard(width=19,height=19, col_major=true)]
	#[derive(Default, BitboardDebug, BitboardDisplay, Hash)]
	pub struct GobanCol;

	#[bitboard(width=19,height=19, col_major=false)]
	#[derive(Default, BitboardDebug, BitboardDisplay, Hash)]
	pub struct Goban;
	


	fn from_indices(indices: &[u16]) -> Goban {
		let mut bb = Goban::empty();
		for &i in indices {
			bb.set_at_index(i as usize);
		}
		bb
	}

	#[test]
	fn test_goban_vertical() {
		println!("GobanCol::H_OFFSET: {}", GobanCol::H_OFFSET);
		println!("GobanCol::V_OFFSET: {}", GobanCol::V_OFFSET);
		println!("GobanCol::DIAG_DEC_OFFSET: {}", GobanCol::DIAG_DEC_OFFSET);
		println!("GobanCol::DIAG_INC_OFFSET: {}", GobanCol::DIAG_INC_OFFSET);
		println!("Goban::H_OFFSET: {}", Goban::H_OFFSET);
		println!("Goban::V_OFFSET: {}", Goban::V_OFFSET);
		println!("Goban::DIAG_DEC_OFFSET: {}", Goban::DIAG_DEC_OFFSET);
		println!("Goban::DIAG_INC_OFFSET: {}", Goban::DIAG_INC_OFFSET);
		let bb = from_indices(&[0, 19, 19*2, 19*3, 19*4]);
		println!("{bb}");
		assert!(bb.has_n_aligned_vertical(5), "Vertical 5-align not detected");
		
		let bb = from_indices(&[9, 9+19, 9+19*2, 9+19*3, 9+19*4]);
		println!("{bb}");
		assert!(bb.has_n_aligned(5), "Vertical 5-align not detected");

		let bb = from_indices(&[18, 18+19, 18+19*2, 18+19*3, 18+19*4]);
		println!("{bb}");
		assert!(bb.has_n_aligned_vertical(5), "Vertical 5-align not detected");
		
		let bb = from_indices(&[100, 100+19, 100+19*2, 100+19*3, 100+19*4]);
		println!("{bb}");
		assert!(bb.has_n_aligned(5), "Vertical 5-align not detected");

		let bleed = from_indices(&[360, 360-19, 360-2*19, 18]); 
		println!("{bleed}");
		assert!(!bleed.has_n_aligned(4), "Vertical bleeding");
	}

	#[test]
	fn test_goban_horizontal() {
		// Colonne 0 : 0, 1, 2, 3...
		let bb = from_indices(&[10, 11, 12, 13]);
		assert!(bb.has_n_aligned_horizontal(4), "Horizontal 4-align not detected");

		let bleed = from_indices(&[16, 17, 18, 19]);
		assert!(!bleed.has_n_aligned_horizontal(4), "Bleeding horizontal not detected");
	}

	#[test]
	fn test_goban_diagonals() {
		let bb_inc = from_indices(&[0, 20, 40, 60]);
		println!("{}", bb_inc);
		assert!(bb_inc.has_n_aligned_diag_inc(4), "Diagonal INC (/) not detected");
		assert!(!bb_inc.has_n_aligned_diag_dec(4), "Diagonal DEC (\\) incorrectly detected");

		let bb_dec = from_indices(&[7, 7+18, 7+2*18, 7+3*18]);
		println!("{}", bb_dec);
		assert!(bb_dec.has_n_aligned_diag_dec(4), "Diagonal DEC (\\) not detected");
		assert!(!bb_dec.has_n_aligned_diag_inc(4), "Diagonal INC (/) incorrectly detected");
	}

	#[test]
	fn test_goban_edge_cases() {
		let bb = from_indices(&[0, 1, 19, 20]);
		assert!(!bb.has_n_aligned(4), "4-alignment detected on a 2x2 square");

		let full_col = Goban::WEST_BORDER;
		assert!(full_col.has_n_aligned(19));
		assert!(!full_col.has_n_aligned(20));
		assert!(Goban::FULL.has_n_aligned(19));
		assert!(!Goban::FULL.has_n_aligned(20));
		assert!(Goban::EMPTY.has_n_aligned(0));
		assert!(!Goban::EMPTY.has_n_aligned(1));
	}
}