
use bitboard::{BitIter, Bitboard, bitboard_table};
use bitboard_proc_macro::{BitboardDebug, BitboardDisplay, BitboardMask, bitboard};

#[bitboard(width=7,height=7)]
#[derive(Default, BitboardDebug, BitboardDisplay, Hash)]
pub struct Bitboard7x7;

#[bitboard(width=8,height=8)]
#[repr(transparent)]
#[derive(Default, BitboardDebug, BitboardDisplay, Hash)]
pub struct Bitboard8x8;
bitboard_table!(RAY_N, ray_n, ray_n_mask, Bitboard8x8, Bitboard8x8::generate_ray_n_table());
bitboard_table!(RAY_S, ray_s, ray_s_mask, Bitboard8x8, Bitboard8x8::generate_ray_s_table());
bitboard_table!(RAY_W, ray_w, ray_w_mask, Bitboard8x8, Bitboard8x8::generate_ray_w_table());
bitboard_table!(RAY_E, ray_e, ray_e_mask, Bitboard8x8, Bitboard8x8::generate_ray_e_table());
bitboard_table!(RAY_NE, ray_ne, ray_ne_mask, Bitboard8x8, Bitboard8x8::generate_ray_ne_table());
bitboard_table!(RAY_NW, ray_nw, ray_nw_mask, Bitboard8x8, Bitboard8x8::generate_ray_nw_table());
bitboard_table!(RAY_SE, ray_se, ray_se_mask, Bitboard8x8, Bitboard8x8::generate_ray_se_table());
bitboard_table!(RAY_SW, ray_sw, ray_sw_mask, Bitboard8x8, Bitboard8x8::generate_ray_sw_table());
bitboard_table!(DIAG_INC, diag_inc, diag_inc_mask, Bitboard8x8, Bitboard8x8::generate_diag_inc_table());
bitboard_table!(DIAG_DEC, diag_dec, diag_dec_mask, Bitboard8x8, Bitboard8x8::generate_diag_dec_table());
bitboard_table!(NEIGHBORS_8, neighbors_8, neighbors_8_mask, Bitboard8x8, Bitboard8x8::generate_neighbors_8_table());

#[bitboard(width=8,height=8, col_major=true)]
#[derive(Default, BitboardDebug, BitboardDisplay)]
struct Bitboard8x8ColMajor;

#[bitboard(width=10,height=10)]
#[repr(transparent)]
#[derive(Default, BitboardDebug, BitboardDisplay, Hash)]
pub struct Bitboard10x10;
bitboard_table!(RAY_N, ray_n, ray_n_mask, Bitboard10x10, Bitboard10x10::generate_ray_n_table());
bitboard_table!(RAY_S, ray_s, ray_s_mask, Bitboard10x10, Bitboard10x10::generate_ray_s_table());
bitboard_table!(RAY_W, ray_w, ray_w_mask, Bitboard10x10, Bitboard10x10::generate_ray_w_table());
bitboard_table!(RAY_E, ray_e, ray_e_mask, Bitboard10x10, Bitboard10x10::generate_ray_e_table());
bitboard_table!(RAY_NE, ray_ne, ray_ne_mask, Bitboard10x10, Bitboard10x10::generate_ray_ne_table());
bitboard_table!(RAY_NW, ray_nw, ray_nw_mask, Bitboard10x10, Bitboard10x10::generate_ray_nw_table());
bitboard_table!(RAY_SE, ray_se, ray_se_mask, Bitboard10x10, Bitboard10x10::generate_ray_se_table());
bitboard_table!(RAY_SW, ray_sw, ray_sw_mask, Bitboard10x10, Bitboard10x10::generate_ray_sw_table());
bitboard_table!(DIAG_INC, diag_inc, diag_inc_mask, Bitboard10x10, Bitboard10x10::generate_diag_inc_table());
bitboard_table!(DIAG_DEC, diag_dec, diag_dec_mask, Bitboard10x10, Bitboard10x10::generate_diag_dec_table());
bitboard_table!(NEIGHBORS_8, neighbors_8, neighbors_8_mask, Bitboard10x10, Bitboard10x10::generate_neighbors_8_table());

#[bitboard(width=5,height=2, col_major=false)]
#[derive(Default, BitboardDebug, BitboardDisplay)]
struct Test;


//impl Test {
//    pub const fn has_alignment_h<const N: usize>(&self) -> bool { ... }
//    pub const fn has_alignment_v<const N: usize>(&self) -> bool { ... }
//    pub const fn has_alignment_diag_inc<const N: usize>(&self) -> bool { ... }
//    pub const fn has_alignment_diag_dec<const N: usize>(&self) -> bool { ... }
//
//    pub const fn has_alignment<const N: usize>(&self) -> bool {
//        self.has_alignment_h::<N>()
//            || self.has_alignment_v::<N>()
//            || self.has_alignment_diag_inc::<N>()
//            || self.has_alignment_diag_dec::<N>()
//    }
//}
#[bitboard(width=15,height=80, col_major=true)]
#[derive(BitboardDebug, BitboardMask, BitboardDisplay)]
struct Test2;


#[test]
fn test_display() {
	let test=Test::default();
	println!("{}", test);
	println!("{:?}", test);
	let test=Test(Test::index_from_coords(0, 1) as u16);
	println!("{}", test);
	println!("{:?}", test);
	let test=Test((1<<12)+(1<<5));
	println!("{}", test);
	println!("{:?}", test);
	println!("{}", test^Test::from_storage(1));
	let test=Test2::new();
	println!("{}", test);
	let test=Bitboard8x8::CENTER;
	println!("{}", test);
	let test=Bitboard8x8::BORDER;
	println!("{}", test);
	let test=Bitboard8x8::WEST_BORDER;
	println!("{}", test);
	let test=Bitboard8x8::EAST_BORDER;
	println!("{}", test);
	let test=Bitboard8x8::SOUTH_BORDER;
	println!("{}", test);
	println!("{:?}", test);
	let test=Bitboard8x8::NORTH_BORDER;
	println!("{}", test);
	println!("{:?}", test);
	let test=Bitboard8x8::NORTH;
	println!("{}", test);
	println!("{:?}", test);
	let test=Bitboard8x8::SOUTH;
	println!("{}", test);
	println!("{:?}", test);
	let test=Bitboard8x8::WEST;
	println!("{}", test);
	println!("{:?}", test);
	let test=Bitboard8x8::EAST;
	println!("{}", test);
	println!("{:?}", test);
	//println!("{:?}", test);
	let test = Bitboard7x7::from_storage(0b1011000);
	for idx in test.iter_bits() {
		println!("bit actif: {}", idx);
	}
	
}
#[test]
fn test_bitboard8x8() {
	assert_eq!(Bitboard8x8::index_from_coords(0, 1), 8);
	assert_eq!(Bitboard8x8ColMajor::H_OFFSET, 8);
	assert_eq!(Bitboard8x8ColMajor::V_OFFSET, 1);
	assert_eq!(Bitboard8x8ColMajor::DIAG_DEC_OFFSET, 9);
	assert_eq!(Bitboard8x8ColMajor::DIAG_INC_OFFSET, 7);
	assert_eq!(Bitboard8x8::H_OFFSET, 1);
	assert_eq!(Bitboard8x8::V_OFFSET, 8);
	let test=Bitboard8x8::from_storage(1<<Bitboard8x8::index_from_coords(0, 1));
	
	assert_eq!(test.storage(), 0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000000);
	let test=Bitboard8x8::from_coords(0, 1);
	assert_eq!(test.storage(), 0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000000);
	let test=Bitboard8x8::from_index(8);
	assert_eq!(test.storage(), 0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000000);
	
	let test=Bitboard8x8::SOUTH_BORDER;
	let repr=format!("{:?}", test);
	assert_eq!(repr, "bitboard::Bitboard8x8(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111)".to_string());
	assert_eq!(test.storage(), 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111);
	
	let test=Bitboard8x8::NORTH_BORDER;
	assert_eq!(test.storage(), 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000);
	
	let test=Bitboard8x8::WEST_BORDER;
	assert_eq!(test.storage(), 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001);
	
	let test=Bitboard8x8::EAST_BORDER;
	assert_eq!(test.storage(), 0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000);
	
	let test=Bitboard8x8::CENTER;
	assert_eq!(test.storage(), 0b00000000_00000000_00000000_00011000_00011000_00000000_00000000_00000000);
	
	let test=Bitboard8x8::SOUTH;
	assert_eq!(test.storage(), 0b00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111);

	let test=Bitboard8x8::NORTH;
	assert_eq!(test.storage(), 0b11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000);

	let test=Bitboard8x8::WEST;
	assert_eq!(test.storage(), 0b00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111);

	let test=Bitboard8x8::EAST;
	assert_eq!(test.storage(), !0b00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111);

	let test = Bitboard8x8::FULL;
	for i in 0..8 {
		assert_eq!(0xFF, test.extract_row(i));
	}
	for i in 0..8 {
		assert_eq!(0xFF, test.extract_col(i));
	}
	for i in 0..8 {
		assert_eq!(0xFF >> (7-i), test.extract_diag_dec(i));
	}
	for i in 0..8 {
		assert_eq!(0xFF >> i, test.extract_diag_inc(i));
	}
	println!("{}", test.ray_ne(44));
	println!("{}", test.ray_nw(44));
	println!("{}", test.ray_se(44));
	println!("{}", test.ray_nw(44));
	println!("{}", test.ray_n(44));
	println!("{}", test.ray_s(44));
	println!("{}", test.ray_e(44));
	println!("{}", test.ray_w(44));
	println!("{}", Bitboard8x8::DIAG_INC[0]);
	println!("{}", Bitboard8x8::DIAG_DEC[0]);
	println!("{}", Bitboard8x8::DIAG_INC[1]);
	println!("{}", Bitboard8x8::DIAG_DEC[1]);
	println!("{}", Bitboard8x8::DIAG_INC[3]);
	println!("{}", Bitboard8x8::DIAG_DEC[3]);
	
	let test = Bitboard8x8::SOUTH_BORDER;
	assert_eq!(0, test.extract_row(1));
	
}
#[test]
fn test_bitboard7x7() {
	assert_eq!(Bitboard7x7::index_from_coords(0, 1), 7);
	assert_eq!(Bitboard7x7::H_OFFSET, 1);
	assert_eq!(Bitboard7x7::V_OFFSET, 7);
	assert_eq!(Bitboard7x7::DIAG_DEC_OFFSET, 8);
	assert_eq!(Bitboard7x7::DIAG_INC_OFFSET, 6);
	let test=Bitboard7x7::from_storage(1<<Bitboard7x7::index_from_coords(0, 1));
	
	assert_eq!(test.storage(), 0b0000000_0000000_0000000_0000000_0000000_0000001_0000000);
	let test=Bitboard7x7::from_coords(0, 1);
	assert_eq!(test.storage(), 0b0000000_0000000_0000000_0000000_0000000_0000001_0000000);
	let test=Bitboard7x7::from_index(7);
	assert_eq!(test.storage(), 0b0000000_0000000_0000000_0000000_0000000_0000001_0000000);
	
	let test=Bitboard7x7::SOUTH_BORDER;
	let repr=format!("{:?}", test);
	assert_eq!(repr, "bitboard::Bitboard7x7(0b0000000_0000000_0000000_0000000_0000000_0000000_1111111)".to_string());
	assert_eq!(test.storage(), 0b0000000_0000000_0000000_0000000_0000000_0000000_1111111);
	
	let test=Bitboard7x7::NORTH_BORDER;
	assert_eq!(test.storage(), 0b1111111_0000000_0000000_0000000_0000000_0000000_0000000);
	
	let test=Bitboard7x7::WEST_BORDER;
	assert_eq!(test.storage(), 0b0000001_0000001_0000001_0000001_0000001_0000001_0000001);
	
	let test=Bitboard7x7::EAST_BORDER;
	assert_eq!(test.storage(), 0b1000000_1000000_1000000_1000000_1000000_1000000_1000000);
	
	let test=Bitboard7x7::CENTER;
	assert_eq!(test.storage(), 0b0000000_0000000_0000000_0001000_0000000_0000000_0000000);
	
	let test=Bitboard7x7::SOUTH;
	assert_eq!(test.storage(), 0b0000000_0000000_0000000_0000000_1111111_1111111_1111111);

	let test=Bitboard7x7::NORTH;
	assert_eq!(test.storage(), 0b1111111_1111111_1111111_0000000_0000000_0000000_0000000);

	let test=Bitboard7x7::WEST;
	assert_eq!(test.storage(), 0b0000111_0000111_0000111_0000111_0000111_0000111_0000111);

	let test=Bitboard7x7::EAST;
	assert_eq!(test.storage(), 0b1110000_1110000_1110000_1110000_1110000_1110000_1110000);

	let test = Bitboard7x7::FULL;
	for i in 0..7 {
		assert_eq!(0x7F, test.extract_row(i));
	}
	for i in 0..7 {
		assert_eq!(0x7F, test.extract_col(i));
	}

	let test = Bitboard7x7::SOUTH_BORDER;
	assert_eq!(0, test.extract_row(1));
	let test  = Bitboard7x7::EVEN_SQUARES;
	println!("{}", test);
}
#[test]
fn test_bitboard10x10() {
	assert_eq!(Bitboard10x10::index_from_coords(0, 1), 10);
	assert_eq!(Bitboard10x10::H_OFFSET, 1);
	assert_eq!(Bitboard10x10::V_OFFSET, 10);
	assert_eq!(Bitboard10x10::DIAG_DEC_OFFSET, 11);
	assert_eq!(Bitboard10x10::DIAG_INC_OFFSET, 9);

	let test = Bitboard10x10::from_storage(1 << Bitboard10x10::index_from_coords(0, 1));
	assert_eq!(test.storage(),
		0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000001_0000000000u128
	);

	let test = Bitboard10x10::from_coords(0, 1);
	assert_eq!(test.storage(),
		0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000001_0000000000u128
	);

	let test = Bitboard10x10::from_index(10);
	assert_eq!(test.storage(),
		0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000001_0000000000u128
	);

	let test = Bitboard10x10::SOUTH_BORDER;
	let repr = format!("{:?}", test);
	assert_eq!(
		repr,
		"static_bitboard::Bitboard10x10(0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_1111111111)"
	);
	assert_eq!(
		test.storage(),
		0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_1111111111u128
	);

	let test = Bitboard10x10::NORTH_BORDER;
	assert_eq!(
		test.storage(),
		0b1111111111_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000_0000000000u128
	);

	let test = Bitboard10x10::WEST_BORDER;
	assert_eq!(
		test.storage(),
		0b0000000001_0000000001_0000000001_0000000001_0000000001_0000000001_0000000001_0000000001_0000000001_0000000001u128
	);

	let test = Bitboard10x10::EAST_BORDER;
	assert_eq!(
		test.storage(),
		0b1000000000_1000000000_1000000000_1000000000_1000000000_1000000000_1000000000_1000000000_1000000000_1000000000u128
	);

	let test = Bitboard10x10::CENTER;
	assert_eq!(
		test.storage(),
		0b0000000000_0000000000_0000000000_0000000000_0000110000_0000110000_0000000000_0000000000_0000000000_0000000000u128
	);

	let test = Bitboard10x10::SOUTH;
	assert_eq!(
		test.storage(),
		0b0000000000_0000000000_0000000000_0000000000_0000000000_1111111111_1111111111_1111111111_1111111111_1111111111u128
	);

	let test = Bitboard10x10::NORTH;
	assert_eq!(
		test.storage(),
		0b1111111111_1111111111_1111111111_1111111111_1111111111_0000000000_0000000000_0000000000_0000000000_0000000000u128
	);

	let test = Bitboard10x10::WEST;
	assert_eq!(
		test.storage(),
		0b0000011111_0000011111_0000011111_0000011111_0000011111_0000011111_0000011111_0000011111_0000011111_0000011111u128
	);

	let test = Bitboard10x10::EAST;
	assert_eq!(
		test.storage(),
		0b1111100000_1111100000_1111100000_1111100000_1111100000_1111100000_1111100000_1111100000_1111100000_1111100000u128
	);

	// FULL
	let test = Bitboard10x10::FULL;
	for i in 0..10 {
		assert_eq!(0b1111111111, test.extract_row(i));
	}
	for i in 0..10 {
		assert_eq!(0b1111111111, test.extract_col(i));
	}

	// SOUTH_BORDER → row 1 = 0
	let test = Bitboard10x10::SOUTH_BORDER;
	assert_eq!(0, test.extract_row(1));

	let test = Bitboard10x10::EVEN_SQUARES;
	assert_eq!(0b0101010101, test.extract_row(0));
	assert_eq!(0b0101010101, test.extract_col(2));
	assert_eq!(0b1010101010, test.extract_col(3));

	let test = Bitboard10x10::ODD_SQUARES;
	assert_eq!(0b1010101010, test.extract_col(2));
	assert_eq!(0b1010101010, test.extract_row(0));

	let mut test = Bitboard10x10::EVEN_SQUARES;
	test.insert_col(0, 0);
	println!("{}", test);
	assert_eq!(0, test.extract_col(0));
	test.insert_row(0, 0);
	println!("{}", test);
	assert_eq!(0, test.extract_row(0));
	test.insert_col(0, Bitboard10x10::FULL.storage());
	println!("{}", test);
	assert_eq!((1 << 10) - 1, test.extract_col(0)); // 10 bits à 1
	test.insert_row(0, Bitboard10x10::FULL.storage());
	println!("{}", test);
	assert_eq!((1 << 10) - 1, test.extract_row(0));

	
	
	let test = Bitboard10x10::EVEN_SQUARES;
	println!("{}", test >> Bitboard10x10::WIDTH);
	println!("{}", test << Bitboard10x10::WIDTH);
	println!("{}", (test >> 1usize) & !Bitboard10x10::EAST_BORDER);
	println!("{}", (test << 1usize) & !Bitboard10x10::WEST_BORDER);

	let mut test = Bitboard10x10::EVEN_SQUARES;
	test=test.shift_s();
	println!("{}", test);
	
	let mut test = Bitboard10x10::EVEN_SQUARES;
	test=test.shift_n();
	println!("{}", test);
	
	let mut test = Bitboard10x10::EVEN_SQUARES;
	test=test.shift_e();
	println!("{}", test);
	
	
	let test = Bitboard10x10::EVEN_SQUARES;
	println!("{}", test.neighbors_8(Bitboard10x10::index_from_coords(3, 7)));

	let test = Bitboard10x10::FULL;
	println!("{}", test.diag_inc(Bitboard10x10::index_from_coords(3, 7)));

	let test = Bitboard10x10::ODD_SQUARES;
	println!("{}", test.diag_inc(Bitboard10x10::index_from_coords(3, 7)));

	let test = Bitboard10x10::ODD_SQUARES;
	println!("{}", test.ray_w(Bitboard10x10::index_from_coords(3, 7)));
	println!("{:010b}", test.ray_w(Bitboard10x10::index_from_coords(3, 7)).extract_row(7));

	let test = Bitboard10x10::FULL;
	println!("{}", test.ray_se(Bitboard10x10::index_from_coords(3, 7)));

	let test = Bitboard10x10::FULL;
	println!("{}", test.ray_ne(Bitboard10x10::index_from_coords(3, 7)));
}

