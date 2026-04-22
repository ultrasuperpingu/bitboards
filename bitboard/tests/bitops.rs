use bitboard::Bitboard;
use bitboard_proc_macro::{BitboardDebug, BitboardDisplay, bitboard};

#[bitboard(width = 7, height = 7)]
#[derive(Default, BitboardDebug, BitboardDisplay, Hash)]
pub struct Bitboard7x7;

#[bitboard(width = 8, height = 8)]
#[repr(transparent)]
#[derive(Default, BitboardDebug, BitboardDisplay, Hash)]
pub struct Bitboard8x8;

#[bitboard(width = 8, height = 8, col_major = true)]
#[derive(Default, BitboardDebug, BitboardDisplay)]
struct Bitboard8x8ColMajor;

#[bitboard(width = 10, height = 10)]
#[repr(transparent)]
#[derive(Default, BitboardDebug, BitboardDisplay, Hash)]
pub struct Bitboard10x10;

#[bitboard(width = 12, height = 12, col_major = true)]
#[derive(BitboardDebug, BitboardDisplay)]
struct Bitboard12x12;
macro_rules! bitop_test {
	($T:ty) => {{
		let mut a = <$T>::empty();
		let mut b = <$T>::empty();

		a.set(1, 1);
		a.set(2, 2);

		b.set(2, 2);
		b.set(3, 3);

		// AND
		let c = a.clone() & b.clone();
		let mut a2 = a.clone();
		a2 &= b.clone();
		assert_eq!(c, a2);

		// OR
		let c = a.clone() | b.clone();
		let mut a2 = a.clone();
		a2 |= b.clone();
		assert_eq!(c, a2);

		// XOR
		let c = a.clone() ^ b.clone();
		let mut a2 = a.clone();
		a2 ^= b.clone();
		assert_eq!(c, a2);

		// NOT double negation
		let d = !!a.clone();
		assert_eq!(a, d);

		// bitops const methods
		let mut a_and_b_assign = a.clone();
		a_and_b_assign.and_assign_const(&b.clone());
		assert_eq!(a_and_b_assign, a.clone().and_const(&b));

		let mut a_or_b_assign = a.clone();
		a_or_b_assign.or_assign_const(&b.clone());
		assert_eq!(a_or_b_assign, a.clone().or_const(&b));

		let mut a_xor_b_assign = a.clone();
		a_xor_b_assign.xor_assign_const(&b.clone());
		assert_eq!(a_xor_b_assign, a.clone().xor_const(&b));


		// SHL
		let c = a.clone() << 1usize;
		let mut a2 = a.clone();
		a2 <<= 1;
		assert_eq!(c, a2);
		
		let c = a.clone() << 2u8;
		let mut a2 = a.clone();
		a2 <<= 2;
		assert_eq!(c, a2);

		let c = a.clone().shl_const(2);
		let mut a2 = a.clone();
		a2.shl_assign_const(2);
		assert_eq!(c, a2);

		// SHR
		let c = a.clone() >> 1usize;
		let mut a2 = a.clone();
		a2 >>= 1;
		assert_eq!(c, a2);

		let c = a.clone().shr_const(2);
		let mut a2 = a.clone();
		a2.shr_assign_const(2);
		assert_eq!(c, a2);
	}};
}
#[test]
fn test_bitboard8x8() {
	bitop_test!(Bitboard8x8);
}

#[test]
fn test_bitboard7x7() {
	bitop_test!(Bitboard7x7);
}

#[test]
fn test_bitboard10x10() {
	bitop_test!(Bitboard10x10);
}

#[test]
fn test_bitboard12x12() {
	bitop_test!(Bitboard12x12);
}
