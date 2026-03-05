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

	use crate::tests::Bitboard7x7Col;

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
		println!("Bitboard7x7Col::H_OFFSET: {}", Bitboard7x7Col::H_OFFSET);
		println!("Bitboard7x7Col::V_OFFSET: {}", Bitboard7x7Col::V_OFFSET);
		println!("Bitboard7x7Col::DIAG_DEC_OFFSET: {}", Bitboard7x7Col::DIAG_DEC_OFFSET);
		println!("Bitboard7x7Col::DIAG_INC_OFFSET: {}", Bitboard7x7Col::DIAG_INC_OFFSET);
		println!("Bitboard8x8::H_OFFSET: {}", Bitboard8x8::H_OFFSET);
		println!("Bitboard8x8::V_OFFSET: {}", Bitboard8x8::V_OFFSET);
		println!("Bitboard8x8::DIAG_DEC_OFFSET: {}", Bitboard8x8::DIAG_DEC_OFFSET);
		println!("Bitboard8x8::DIAG_INC_OFFSET: {}", Bitboard8x8::DIAG_INC_OFFSET);
		// En Col Major 8x8, la ligne 0 est : 0, 8, 16, 24, 32, 40, 48, 56
		let bb = from_indices(&[0, 8, 16, 24, 32]);
		println!("{bb}");
		assert!(bb.has_n_aligned_vertical(5), "Horizontal 5-align non détecté");
		
		let bb = from_indices(&[24, 32, 40, 48, 56]);
		println!("{bb}");
		assert!(bb.has_n_aligned(5), "Horizontal 5-align non détecté");

		let bb = from_indices(&[0+7, 8+7, 16+7, 24+7, 32+7]);
		println!("{bb}");
		assert!(bb.has_n_aligned_vertical(5), "Horizontal 5-align non détecté");
		
		let bb = from_indices(&[24+7, 32+7, 40+7, 48+7, 56+7]);
		println!("{bb}");
		assert!(bb.has_n_aligned(5), "Horizontal 5-align non détecté");

		// Test bleeding horizontal : fin de ligne 0 (index 56) vers début ligne 1 (index 1)
		// (Note: l'index dépend de si tu as une ligne sentinelle ou pas, ici on teste pur 8x8)
		let bleed = from_indices(&[40, 48, 56, 1]); 
		assert!(!bleed.has_n_aligned(4), "Bleeding horizontal détecté sur 8x8");
	}

	#[test]
	fn test_8x8_horizontal() {
		// Colonne 0 : 0, 1, 2, 3...
		let bb = from_indices(&[10, 11, 12, 13]);
		assert!(bb.has_n_aligned_horizontal(4), "Vertical 4-align non détecté");

		// Test bleeding vertical : haut d'une colonne (7) vers bas de la suivante (8)
		let bleed = from_indices(&[6, 7, 8, 9]);
		assert!(!bleed.has_n_aligned_horizontal(4), "Bleeding vertical détecté sur 8x8");
	}

	#[test]
	fn test_8x8_diagonals() {
		// INC (/) : +1 col (+8), +1 row (+1) = Offset 9
		// Ex: 0, 9, 18, 27
		let bb_inc = from_indices(&[0, 9, 18, 27]);
		println!("{}", bb_inc);
		assert!(bb_inc.has_n_aligned(4), "Diagonale INC (/) non détectée");

		// DEC (\) : +1 col (+8), -1 row (-1) = Offset 7
		// Ex: 7, 14, 21, 28
		let bb_dec = from_indices(&[7, 14, 21, 28]);
		println!("{}", bb_dec);
		assert!(bb_dec.has_n_aligned(4), "Diagonale DEC (\\) non détectée");
	}

	#[test]
	fn test_8x8_edge_cases() {
		// Test du carré 2x2 (ne doit pas être vu comme un 4-align par erreur)
		let bb = from_indices(&[0, 1, 8, 9]);
		assert!(!bb.has_n_aligned(4), "Faux positif sur un carré 2x2");

		// Ligne complète de 8
		let full_row = from_indices(&[0, 8, 16, 24, 32, 40, 48, 56]);
		assert!(full_row.has_n_aligned(8));
	}
}