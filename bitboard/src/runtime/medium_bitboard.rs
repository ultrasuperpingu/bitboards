use crate::{Bitboard, fmt_bitboard_debug, fmt_bitboard_display, runtime::RuntimeBitboard};

#[derive(Clone, PartialEq, Eq, Copy)]
pub struct MediumBitboard {
	w: u8,
	h: u8,
	col_major: bool,
	bits: u128,
}
impl MediumBitboard {
	pub fn all_subsets(&self) -> Vec<Self>
	{
		let mut subsets = Vec::new();
		let zero = self.empty_with_same_shape();

		let mut subset = zero.clone();

		loop {
			subsets.push(subset.clone());
			subset = Self::new(self.w, self.h, self.col_major, subset.bits.wrapping_sub(self.bits) & self.bits);
			if subset == zero {
				break;
			}
		}

		subsets
	}
}
impl MediumBitboard {
	#[inline(always)]
	pub fn new(w: u8, h: u8, col_major: bool, bits: u128) -> Self {
		assert!(w*h<=128);
		Self { w, h, col_major, bits }
	}
	#[inline(always)]
	pub fn west_border_with_same_shape(&self) -> Self {
		Self::west_border(self.w, self.h, self.col_major)
	}
	#[inline(always)]
	pub fn east_border_with_same_shape(&self) -> Self {
		Self::east_border(self.w, self.h, self.col_major)
	}
	#[inline(always)]
	pub fn north_border_with_same_shape(&self) -> Self {
		Self::north_border(self.w, self.h, self.col_major)
	}
	#[inline(always)]
	pub fn south_border_with_same_shape(&self) -> Self {
		Self::south_border(self.w, self.h, self.col_major)
	}
	#[inline(always)]
	pub fn full_with_same_shape(&self) -> Self {
		Self::full(self.w, self.h, self.col_major)
	}
	#[inline(always)]
	pub fn empty_with_same_shape(&self) -> Self {
		Self::empty(self.w, self.h, self.col_major)
	}
}

impl Default for MediumBitboard {
	#[inline(always)]
	fn default() -> Self {
		Self { w: 8, h: 8, col_major: false, bits: 0 }
	}
}
impl RuntimeBitboard for MediumBitboard {
	#[inline(always)]
	fn new(w: u8, h: u8, col_major: bool, bits: u128) -> Self {
		assert!(w*h<=128);
		Self { w, h, col_major, bits }
	}
	#[inline(always)]
	fn full(w: u8, h: u8, col_major: bool) -> Self {
		let n = w as u32 * h as u32;

		let bits = if n == 128 {
			u128::MAX
		} else {
			(1u128 << n) - 1
		};

		Self::new(w, h, col_major, bits)
	}
	
	#[inline(always)]
	fn empty(w: u8, h: u8, col_major: bool) -> Self {
		Self::new(w, h, col_major, 0)
	}
	#[inline(always)]
	fn borders(w: u8, h: u8, col_major: bool) -> Self {
		Self::west_border(w, h, col_major) | Self::east_border(w, h, col_major) |
		Self::north_border(w, h, col_major) | Self::south_border(w, h, col_major)
	}
	#[inline(always)]
	fn west_border(w: u8, h: u8, col_major: bool) -> Self {
		let mut bits = 0;
		for y in 0..h as usize {
			let idx = y * w as usize;
			bits |= 1 << idx;
		}
		Self::new(w, h, col_major, bits)
	}

	#[inline(always)]
	fn east_border(w: u8, h: u8, col_major: bool) -> Self {
		let mut bits = 0u128;
		let mut v = 1u128 << (w - 1);

		for _ in 0..h {
			bits |= v;
			v <<= w;
		}

		Self::new(w, h, col_major, bits)
	}
	#[inline(always)]
	fn south_border(w: u8, h: u8, col_major: bool) -> Self {
		let bits = if w == 128 {
			u128::MAX
		} else {
			(1u128 << w) - 1
		};

		Self::new(w, h, col_major, bits)
	}

	#[inline(always)]
	fn north_border(w: u8, h: u8, col_major: bool) -> Self {
		let mut bits = if w == 128 {
			u128::MAX
		} else {
			(1u128 << w) - 1
		};

		bits <<= w * (h - 1);

		Self::new(w, h, col_major, bits)
	}
	#[inline(always)]
	fn row_mask(w: u8, h: u8, col_major: bool, y: u8) -> Self {
		if col_major {
			Self::west_border(w,h,col_major) << y
		} else {
			Self{ w, h, col_major, bits: ((1u128 << w) - 1) << (y * w) }
		}
	}
	#[inline(always)]
	fn col_mask(w: u8, h: u8, col_major: bool, x: u8) -> Self {
		if col_major {
			Self{ w, h, col_major, bits: ((1u128 << h) - 1) << (x * h) }
		} else {
			Self::west_border(w,h,col_major) << x
		}
	}
	fn index_from_coords(w: u8, h: u8, col_major: bool, x: u8, y: u8) -> usize {
		if col_major {
			x as usize * h as usize + y as usize
		} else {
			y as usize * w as usize + x as usize
		}
	}
	
	fn coords_from_index(w: u8, h: u8, col_major: bool, i: usize) -> (u8, u8) {
		if col_major {
			((i / h as usize) as u8, (i % h as usize) as u8)
		} else {
			((i % w as usize) as u8, (i / w as usize) as u8)
		}
	}
	
	fn is_in_bounds(w: u8, h: u8, x: u8, y: u8) -> bool {
		x < w && y < h
	}
	
	fn is_index_in_bounds(w: u8, h: u8, i: usize) -> bool {
		i < w as usize * h as usize
	}
	#[inline(always)]
	fn full_with_same_shape(&self) -> Self {
		Self::full(self.w, self.h, self.col_major)
	}

	#[inline(always)]
	fn empty_with_same_shape(&self) -> Self {
		Self::empty(self.w, self.h, self.col_major)
	}

	#[inline(always)]
	fn west_border_with_same_shape(&self) -> Self {
		Self::west_border(self.w, self.h, self.col_major)
	}

	#[inline(always)]
	fn east_border_with_same_shape(&self) -> Self {
		Self::east_border(self.w, self.h, self.col_major)
	}

	#[inline(always)]
	fn north_border_with_same_shape(&self) -> Self {
		Self::north_border(self.w, self.h, self.col_major)
	}

	#[inline(always)]
	fn south_border_with_same_shape(&self) -> Self {
		Self::south_border(self.w, self.h, self.col_major)
	}

	#[inline(always)]
	fn row_mask_with_same_shape(&self, y: u8) -> Self {
		Self::row_mask(self.w, self.h, self.col_major, y)
	}
	#[inline(always)]
	fn col_mask_with_same_shape(&self, x: u8) -> Self {
		Self::col_mask(self.w, self.h, self.col_major, x)
	}

	#[inline(always)]
	fn index_from_coords_with_same_shape(&self, x: u8, y: u8) -> usize {
		Self::index_from_coords(self.w, self.h, self.col_major, x, y)
	}
	
	#[inline(always)]
	fn coords_from_index_with_same_shape(&self, i: usize) -> (u8, u8) {
		Self::coords_from_index(self.w, self.h, self.col_major, i)
	}
	
	#[inline(always)]
	fn is_in_bounds_with_same_shape(&self, x: u8, y: u8) -> bool {
		Self::is_in_bounds(self.w, self.h, x, y)
	}
	
	#[inline(always)]
	fn is_index_in_bounds_with_same_shape(&self, i: usize) -> bool {
		Self::is_index_in_bounds(self.w, self.h, i)
	}
}
impl Bitboard for MediumBitboard {
	type Storage=u128;
	#[inline(always)]
	fn width(&self) -> u8 { self.w }
	#[inline(always)]
	fn height(&self) -> u8 { self.h }
	#[inline(always)]
	fn col_major(&self) -> bool { self.col_major }


	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.bits == 0
	}
	#[inline(always)]
	fn count(&self) -> u32 {
		self.bits.count_ones()
	}
	#[inline(always)]
	fn intersects(&self, other: &Self) -> bool {
		self.bits & other.bits != 0
	}
	#[inline(always)]
	fn any(&self) -> bool {
		self.bits != 0
	}
	#[inline(always)]
	fn storage(&self) -> &Self::Storage {
		&self.bits
	}
	#[inline(always)]
	fn storage_mut(&mut self) -> &mut Self::Storage {
		&mut self.bits
	}
	
	#[inline(always)]
	fn get_at_index(&self, idx: usize) -> bool {
		(self.bits & (1 << idx)) != 0
	}
	#[inline(always)]
	fn set_value_at_index(&mut self, idx: usize, val: bool) {
		if val {
			self.bits |= 1 << idx;
		} else {
			self.bits &= !(1 << idx);
		}
	}
	#[inline(always)]
	fn set_at_index(&mut self, idx: usize) {
		self.bits |= 1 << idx;
	}
	#[inline(always)]
	fn reset_at_index(&mut self, idx: usize) {
		self.bits &= !(1 << idx);
	}
	#[inline(always)]
	fn toggle_at_index(&mut self, idx: usize) {
		self.bits ^= 1 << idx;
	}
	#[inline(always)]
	fn get(&self, x: u8, y: u8) -> bool {
		self.get_at_index(self.index_from_coords_with_same_shape(x, y))
	}

	#[inline(always)]
	fn set_value(&mut self, x: u8, y: u8, val: bool) {
		self.set_value_at_index(self.index_from_coords_with_same_shape(x, y), val)
	}
	#[inline(always)]
	fn set(&mut self, x: u8, y: u8) {
		self.set_at_index(self.index_from_coords_with_same_shape(x, y))
	}
	#[inline(always)]
	fn reset(&mut self, x: u8, y: u8) {
		self.reset_at_index(self.index_from_coords_with_same_shape(x, y))
	}
	
	#[inline(always)]
	fn flipped(&self) -> Self {
		Self{ w: self.w, h: self.h, col_major: self.col_major, bits: !self.bits }
	}
	#[inline]
	fn lsb(&self) -> u32 {
		self.bits.trailing_zeros()
	}
	#[inline]
	fn pop_lsb(&mut self) -> u32 {
		let low = self.bits as u64;

		if low != 0 {
			let idx = low.trailing_zeros();
			self.bits &= self.bits - 1;
			return idx;
		}

		let high = (self.bits >> 64) as u64;
		let idx = high.trailing_zeros();
		self.bits &= self.bits - 1;
		idx + 64
	}

	#[inline]
	fn pext(&self, mask: &Self) -> Self::Storage {
		#[cfg(target_feature = "bmi2")]
		unsafe {
			let lo  = self.bits as u64;
			let hi  = (self.bits >> 64) as u64;

			let mlo = mask.bits as u64;
			let mhi = (mask.bits >> 64) as u64;

			let lo_res = std::arch::x86_64::_pext_u64(lo, mlo);
			let hi_res = std::arch::x86_64::_pext_u64(hi, mhi);

			let shift = mlo.count_ones();

			((hi_res as u128) << shift) | (lo_res as u128)
		}
		#[cfg(not(target_feature = "bmi2"))]
		{
			let mut res: u128 = 0;
			let mut bit: u128 = 1;
			let mut m = mask.bits;
			let mut v = self.bits;

			while m != 0 {
				if m & 1 != 0 {
					if v & 1 != 0 {
						res |= bit;
					}
					bit <<= 1;
				}
				m >>= 1;
				v >>= 1;
			}
			res
		}
	}
	#[inline]
	fn pdep(&self, compressed: Self::Storage) -> Self {
		#[cfg(target_feature = "bmi2")]
		unsafe {
			let src = compressed;

			let lo_mask = self.bits as u64;
			let hi_mask = (self.bits >> 64) as u64;

			let lo_count = lo_mask.count_ones();

			let src_lo = src as u64;
			let src_hi = (src >> lo_count) as u64;

			let lo_res = std::arch::x86_64::_pdep_u64(src_lo, lo_mask);
			let hi_res = std::arch::x86_64::_pdep_u64(src_hi, hi_mask);

			Self::new(self.w, self.h, self.col_major, (lo_res as u128) | ((hi_res as u128) << 64))
		}
		#[cfg(not(target_feature = "bmi2"))]
		{
			
			// fallback
			let mut res: u128 = 0;
			let mut bit: u128 = 1;
			let mut m = compressed;
			let mut v = self.bits;

			while m != 0 {
				if m & 1 != 0 {
					if v & 1 != 0 {
						res |= bit;
					}
					v >>= 1;
				}
				bit <<= 1;
				m >>= 1;
			}
			Self::new(self.w, self.h, self.col_major, res)
		}
	}

	#[inline(always)]
	fn extract_row(&self, y: u8) -> Self::Storage {
		let mask = self.row_mask_with_same_shape(y);
		self.pext(&mask)
	}
	#[inline(always)]
	fn extract_col(&self, x: u8) -> Self::Storage {
		let mask = self.col_mask_with_same_shape(x);
		self.pext(&mask)
	}
	#[inline(always)]
	fn insert_row(&mut self, y: u8, row_bits: Self::Storage) {
		let mask = self.row_mask_with_same_shape(y);

		let new_row = mask.pdep(row_bits);
		let cleared = *self & !mask;

		*self = cleared | new_row;
	}

	#[inline(always)]
	fn insert_col(&mut self,x: u8, col_bits: Self::Storage) {
		let mask = self.col_mask_with_same_shape(x);

		let new_row = mask.pdep(col_bits);
		let cleared = *self & !mask;

		*self = cleared | new_row;
	}
	
}
impl std::ops::BitAnd for MediumBitboard {
	type Output = Self;
	#[inline(always)]
	fn bitand(mut self, rhs: Self) -> Self {
		self.bits &= rhs.bits;
		self
	}
}

impl std::ops::BitOr for MediumBitboard {
	type Output = Self;
	#[inline(always)]
	fn bitor(mut self, rhs: Self) -> Self {
		self.bits |= rhs.bits;
		self
	}
}

impl std::ops::BitXor for MediumBitboard {
	type Output = Self;
	#[inline(always)]
	fn bitxor(mut self, rhs: Self) -> Self {
		self.bits ^= rhs.bits;
		self
	}
}
impl std::ops::BitAnd<&Self> for MediumBitboard {
	type Output = Self;

	#[inline(always)]
	fn bitand(mut self, rhs: &Self) -> Self {
		self.bits &= rhs.bits;
		self
	}
}
impl std::ops::BitOr<&Self> for MediumBitboard {
	type Output = Self;

	#[inline(always)]
	fn bitor(mut self, rhs: &Self) -> Self {
		self.bits |= rhs.bits;
		self
	}
}
impl std::ops::BitXor<&Self> for MediumBitboard {
	type Output = Self;

	#[inline(always)]
	fn bitxor(mut self, rhs: &Self) -> Self {
		self.bits ^= rhs.bits;
		self
	}
}

impl std::ops::BitAndAssign for MediumBitboard {
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: Self) {
		self.bits &= rhs.bits;
	}
}

impl std::ops::BitOrAssign for MediumBitboard {
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: Self) {
		self.bits |= rhs.bits;
	}
}

impl std::ops::BitXorAssign for MediumBitboard {
	#[inline(always)]
	fn bitxor_assign(&mut self, rhs: Self) {
		self.bits ^= rhs.bits;
	}
}

impl std::ops::Not for MediumBitboard {
	type Output = Self;

	#[inline(always)]
	fn not(mut self) -> Self {
		self.bits = !self.bits;
		self
	}
}

impl std::ops::Shl<usize> for MediumBitboard {
	type Output = Self;

	#[inline(always)]
	fn shl(mut self, rhs: usize) -> Self {
		self.bits <<= rhs;
		self
	}
}

impl std::ops::Shr<usize> for MediumBitboard {
	type Output = Self;

	#[inline(always)]
	fn shr(mut self, rhs: usize) -> Self {
		self.bits >>= rhs;
		self
	}
}
impl std::ops::ShlAssign<usize> for MediumBitboard {
	#[inline(always)]
	fn shl_assign(&mut self, rhs: usize) {
		self.bits <<= rhs;
	}
}
impl std::ops::ShrAssign<usize> for MediumBitboard {
	#[inline(always)]
	fn shr_assign(&mut self, rhs: usize) {
		self.bits >>= rhs;
	}
}
impl std::ops::Shl<u8> for MediumBitboard {
	type Output = Self;

	#[inline(always)]
	fn shl(mut self, rhs: u8) -> Self {
		self.bits <<= rhs;
		self
	}
}

impl std::ops::Shr<u8> for MediumBitboard {
	type Output = Self;

	#[inline(always)]
	fn shr(mut self, rhs: u8) -> Self {
		self.bits >>= rhs;
		self
	}
}

impl std::fmt::Display for MediumBitboard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		fmt_bitboard_display(self, f)
	}
}

impl std::fmt::Debug for MediumBitboard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		fmt_bitboard_debug(self, std::mem::size_of_val(self.storage()) * 8, f)
	}
}

#[cfg(test)]
mod tests {
	use crate::runtime::RuntimeBitboard;

	use super::MediumBitboard;

	#[test]
	fn test() {
		let bitboard=MediumBitboard::new(13,9,false,0);
		println!("{}", bitboard);
		println!("{}", bitboard.east_border_with_same_shape());
		println!("{}", bitboard.north_border_with_same_shape());
		println!("{}", bitboard.south_border_with_same_shape());
		println!("{}", bitboard.west_border_with_same_shape());
		println!("{:?}", bitboard.full_with_same_shape());
		println!("{}", bitboard.row_mask_with_same_shape(4));
		println!("{}", bitboard.col_mask_with_same_shape(3));
	}
}