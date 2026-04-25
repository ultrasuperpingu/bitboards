
use bitboard::Bitboard;
use bitboard_proc_macro::{BitboardDebug, BitboardDisplay, bitboard};

#[bitboard(width=7,height=7)]
#[derive(Default, BitboardDebug, BitboardDisplay, Hash)]
struct Bitboard7x7;

#[bitboard(width=8,height=8)]
#[repr(transparent)]
#[derive(Default, BitboardDebug, BitboardDisplay, Hash)]
struct Bitboard8x8;

#[bitboard(width=8,height=8, col_major=true)]
#[derive(Default, BitboardDebug, BitboardDisplay)]
struct Bitboard8x8ColMajor;

#[bitboard(width=10,height=10)]
#[repr(transparent)]
#[derive(Default, BitboardDebug, BitboardDisplay, Hash)]
struct Bitboard10x10;

#[bitboard(width=5,height=2, col_major=false)]
#[derive(Default, BitboardDebug, BitboardDisplay)]
struct Test;


#[bitboard(width=12,height=12, col_major=true)]
#[derive(BitboardDebug, BitboardDisplay)]
struct Bitboard12x12;

#[test]
fn test_array_bitboard() {
	let mut test = Bitboard12x12::FULL;
	test=test.shifted_s();
	println!("shifted_s:\n{}", test);
	
	let mut test = Bitboard12x12::FULL;
	test=test.shifted_n();
	println!("shifted_n:\n{}", test);
	
	let mut test = Bitboard12x12::FULL;
	test=test.shifted_e();
	println!("shifted_e:\n{}", test);

	let mut test = Bitboard12x12::FULL;
	test.shift_e_by(5);
	println!("shift_e_by(5):\n{}", test);

	let mut test = Bitboard12x12::FULL;
	test.shift_w_by(5);
	println!("shift_w_by(5):\n{}", test);

	let mut test = Bitboard12x12::FULL;
	test.shift_n_by(5);
	println!("shift_n_by(5):\n{}", test);

	let mut test = Bitboard12x12::FULL;
	test.shift_s_by(5);
	println!("shift_s_by(5):\n{}", test);

	let mut test = Bitboard12x12::FULL;
	test.shift_ne_by(5);
	println!("shift_ne_by(5):\n{}", test);

	let mut test = Bitboard12x12::FULL;
	test.shift_nw_by(5);
	println!("shift_nw_by(5):\n{}", test);

	let mut test = Bitboard12x12::FULL;
	test.shift_se_by(5);
	println!("shift_se_by(5):\n{}", test);

	let mut test = Bitboard12x12::FULL;
	test.shift_sw_by(5);
	println!("shift_sw_by(5):\n{}", test);

	let mut test = Bitboard12x12::FULL;
	test.shift(5,-2);
	println!("shift(5,-2):\n{}", test);

	let mut test = Bitboard12x12::FULL;
	test.shift(-3, -4);
	println!("shift(-3,-4):\n{}", test);

	println!("{}", Bitboard12x12::compute_north_cols_mask(5));
	for x in 0..8 {
		for y in 0..8 {
			let mut b = Bitboard12x12::empty();
			b.set(x, y);

			let r2 = b.shifted(3, 3);
			b.shift_ne_by(3);
			
			assert_eq!(b, r2);
		}
	}
	for x in 0..8 {
		for y in 0..8 {
			let mut b = Bitboard12x12::empty();
			b.set(x, y);

			let r2 = b.shifted(-5, 5);
			b.shift_nw_by(5);
			
			assert_eq!(b, r2);
		}
	}
	for x in 0..8 {
		for y in 0..8 {
			let mut b = Bitboard12x12::empty();
			b.set(x, y);

			let r2 = b.shifted(-1, -1);
			b.shift_sw_by(1);
			
			assert_eq!(b, r2);
		}
	}
	for x in 0..8 {
		for y in 0..8 {
			let mut b = Bitboard12x12::empty();
			b.set(x, y);

			let r2 = b.shifted(2, -2);
			b.shift_se_by(2);
			
			assert_eq!(b, r2);
		}
	}
}
#[test]
fn test_bitboard8x8() {
	let mut test = Bitboard8x8::EVEN_SQUARES;
	test=test.shifted_s();
	println!("{}", test);
	
	let mut test = Bitboard8x8::EVEN_SQUARES;
	test=test.shifted_n();
	println!("{}", test);
	
	let mut test = Bitboard8x8::EVEN_SQUARES;
	test=test.shifted_e();
	println!("{}", test);

	let mut test = Bitboard8x8::EVEN_SQUARES;
	test.shift_e_by(5);
	println!("{}", test);

	let mut test = Bitboard8x8::EVEN_SQUARES;
	test.shift_w_by(5);
	println!("{}", test);

	let mut test = Bitboard8x8::EVEN_SQUARES;
	test.shift_n_by(5);
	println!("{}", test);

	let mut test = Bitboard8x8::EVEN_SQUARES;
	test.shift_s_by(5);
	println!("{}", test);

	let mut test = Bitboard8x8::EVEN_SQUARES;
	test.shift_ne_by(5);
	println!("{}", test);

	let mut test = Bitboard8x8::EVEN_SQUARES;
	test.shift_se_by(5);
	println!("{}", test);
	
	let mut test = Bitboard8x8::EVEN_SQUARES;
	test.shift(5,2);
	println!("{}", test);

	let mut test = Bitboard8x8::EVEN_SQUARES;
	test.shift(-3, -4);
	println!("{}", test);

	let mut test = Bitboard8x8::EVEN_SQUARES;
	test.shift(3, -7);
	println!("{}", test);

	let mut test = Bitboard8x8::EVEN_SQUARES;
	test.shift(-3, 1);
	println!("{}", test);
}
#[test]
fn test_bitboard10x10() {
	let mut test = Bitboard10x10::EVEN_SQUARES;
	println!("{}", test);
	test=test.shifted_s();
	println!("shifted_s\n{}\n{:X}", test, test.0);
	assert_eq!(test.0, 0x2AA556AA556AA556AA556AA);
	
	let mut test = Bitboard10x10::EVEN_SQUARES;
	test=test.shifted_n();
	println!("shifted_n\n{}\n{:X}", test, test.0);
	assert_eq!(test.0, 0x2AA556AA556AA556AA556AA55400);
	
	let mut test = Bitboard10x10::EVEN_SQUARES;
	test=test.shifted_e();
	println!("shifted_e\n{}\n{:X}", test, test.0);
	assert_eq!(test.0, 0x552AA552AA552AA552AA552AA);

	let mut test = Bitboard10x10::EVEN_SQUARES;
	test.shift_e_by(5);
	println!("shift_e_by\n{}\n{:X}", test, test.0);
	assert_eq!(test.0, 0x502A0502A0502A0502A0502A0);

	let mut test = Bitboard10x10::EVEN_SQUARES;
	test.shift_w_by(5);
	println!("shift_w_by\n{}\n{:X}", test, test.0);
	assert_eq!(test.0, 0x540A0540A0540A0540A0540A);

	let mut test = Bitboard10x10::EVEN_SQUARES;
	test.shift_n_by(5);
	println!("shift_n_by\n{}\n{:X}", test, test.0);
	assert_eq!(test.0, 0xAA556AA556AA556AA554000000000000);

	let mut test = Bitboard10x10::EVEN_SQUARES;
	test.shift_s_by(5);
	println!("shift_s_by\n{}\n{:X}", test, test.0);
	assert_eq!(test.0, 0x2AA556AA556AA);

	let mut test = Bitboard10x10::EVEN_SQUARES;
	test.shift_ne_by(5);
	println!("shift_ne_by\n{}\n{:X}", test, test.0);
	assert_eq!(test.0, 0x40A8140A8140A8140A80000000000000);

	let mut test = Bitboard10x10::EVEN_SQUARES;
	test.shift_nw_by(5);
	println!("shift_nw_by\n{}\n{:X}", test, test.0);
	assert_eq!(test.0, 0x15028150281502815028000000000000);

	let mut test = Bitboard10x10::EVEN_SQUARES;
	test.shift_se_by(5);
	println!("shift_se_by\n{}\n{:X}", test, test.0);
	assert_eq!(test.0, 0x140A8140A8140);

	let mut test = Bitboard10x10::EVEN_SQUARES;
	test.shift_sw_by(5);
	println!("shift_sw_by\n{}\n{:X}", test, test.0);
	assert_eq!(test.0, 0x150281502815);

	for x in 0..8 {
		for y in 0..8 {
			let mut b = Bitboard8x8::empty();
			b.set(x, y);

			let r2 = b.shifted(3, 3);
			b.shift_ne_by(3);
			
			assert_eq!(b, r2);
		}
	}
	for x in 0..8 {
		for y in 0..8 {
			let mut b = Bitboard8x8::empty();
			b.set(x, y);

			let r2 = b.shifted(-5, 5);
			b.shift_nw_by(5);
			
			assert_eq!(b, r2);
		}
	}
	for x in 0..8 {
		for y in 0..8 {
			let mut b = Bitboard8x8::empty();
			b.set(x, y);

			let r2 = b.shifted(-1, -1);
			b.shift_sw_by(1);
			
			assert_eq!(b, r2);
		}
	}
	for x in 0..8 {
		for y in 0..8 {
			let mut b = Bitboard8x8::empty();
			b.set(x, y);

			let r2 = b.shifted(2, -2);
			b.shift_se_by(2);
			
			assert_eq!(b, r2);
		}
	}
}
