use crate::{Bitboard, fmt_bitboard_debug, fmt_bitboard_display, runtime::RuntimeBitboard};


#[derive(Clone, PartialEq, Eq)]
pub struct LargeBitboard {
	w: u8,
	h: u8,
	col_major: bool,
	bits: Box<[u64]>,
}
impl RuntimeBitboard for LargeBitboard {

	#[inline(always)]
	fn new(w: u8, h: u8, col_major: bool, bits:Box<[u64]>) -> Self {
		Self { w, h, col_major, bits }
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
	fn full_with_same_shape(&self) -> Self {
		Self::full(self.w, self.h, self.col_major)
	}
	#[inline(always)]
	fn empty_with_same_shape(&self) -> Self {
		Self::empty(self.w, self.h, self.col_major)
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
	#[inline(always)]
	fn borders(w: u8, h: u8, col_major: bool) -> Self {
		Self::west_border(w, h, col_major) | Self::east_border(w, h, col_major) |
		Self::north_border(w, h, col_major) | Self::south_border(w, h, col_major)
	}
	fn west_border(w: u8, h: u8, col_major: bool) -> Self {
		let nb_bits = w as usize * h as usize;
		let nb_words = nb_bits.div_ceil(64);

		let mut bits = vec![0u64; nb_words];

		for y in 0..h {
			let idx = y as usize * w as usize;
			let word = idx / 64;
			let bit  = idx % 64;

			bits[word] |= 1u64 << bit;
		}
		Self::new(w, h, col_major, bits.into_boxed_slice())
	}
	fn east_border(w: u8, h: u8, col_major: bool) -> Self {
		Self::col_mask_with_same_shape(&Self::empty(w, h, col_major), w - 1)
	}
	fn north_border(w: u8, h: u8, col_major: bool) -> Self {
		Self::row_mask_with_same_shape(&Self::empty(w,h,col_major), h - 1)
	}
	fn south_border(w: u8, h: u8, col_major: bool) -> Self {
		Self::row_mask_with_same_shape(&Self::empty(w,h,col_major), 0)
	}

	fn empty(w: u8, h: u8, col_major: bool) -> Self {
		let nb_bits = w as usize * h as usize;
		let nb_words= nb_bits.div_ceil(64);

		Self {
			w,
			h,
			col_major,
			bits: vec![0u64; nb_words].into_boxed_slice(),
		}
	}
	fn full(w: u8, h: u8, col_major: bool) -> Self {
		let nb_bits = w as usize* h as usize;
		let nb_words=nb_bits.div_ceil(64);

		let mut bits = vec![u64::MAX; nb_words];
		let rem = nb_bits % 64;
		if rem != 0 {
			let mask = if rem == 64 { u64::MAX } else { (1u64 << rem) - 1 };
			let last = bits.last_mut().unwrap();
			*last = mask;
		}
		Self::new(w, h, col_major, bits.into_boxed_slice())
	}
	#[inline(always)]
	fn row_mask_with_same_shape(&self, y: u8) -> Self {
		Self::row_mask(self.w, self.h, self.col_major, y)
	}
	#[inline(always)]
	fn col_mask_with_same_shape(&self, x: u8) -> Self {
		Self::col_mask(self.w, self.h, self.col_major, x)
	}
	fn row_mask(w: u8, h: u8, col_major: bool, y: u8) -> Self {
		if col_major {
			Self::west_border(w,h,col_major) << y
		} else {
			let mut row = Self::empty(w, h, col_major);
			row.bits[0]=1;
			row <<= w;
			row -= 1;
			row <<= y * w;
			row
		}
	}
	fn col_mask(w: u8, h: u8, col_major: bool, x: u8) -> Self {
		if col_major {
			let mut col = Self::empty(w, h, col_major);
			col.bits[0]=1;
			col <<= h;
			col -= 1;
			col <<= x * h;
			col
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
}

impl Bitboard for LargeBitboard {
	type Storage=Box<[u64]>;
	#[inline(always)]
	fn width(&self) -> u8 { self.w }
	#[inline(always)]
	fn height(&self) -> u8 { self.h }
	#[inline(always)]
	fn col_major(&self) -> bool { self.col_major }

	#[inline]
	fn is_empty(&self) -> bool {
		for a in self.bits.iter() {
			if *a != 0 {
				return false;
			}
		}
		true
	}
	#[inline]
	fn count(&self) -> u32 {
		let mut counts=0;
		for a in self.bits.iter() {
			counts+=a.count_ones();
		}
		counts
	}
	#[inline]
	fn intersects(&self, other: &Self) -> bool {
		for (a, b) in self.bits.iter().zip(other.bits.iter()) {
			if *a & *b != 0 {
				return false;
			}
		}
		true
	}
	#[inline]
	fn storage(&self) -> &Self::Storage {
		&self.bits
	}
	#[inline]
	fn storage_mut(&mut self) -> &mut Self::Storage {
		&mut self.bits
	}

	#[inline]
	fn get_at_index(&self, idx: usize) -> bool {
		let byte = idx / 64;
		let bit = idx % 64;
		(self.bits[byte] >> bit) & 1 == 1
	}

	#[inline]
	fn set_value_at_index(&mut self, idx: usize, val: bool) {
		let byte = idx / 64;
		let bit = idx % 64;
		if val {
			self.bits[byte] |= 1 << bit;
		} else {
			self.bits[byte] &= !(1 << bit);
		}
	}
	#[inline(always)]
	fn set_at_index(&mut self, idx: usize) {
		let byte = idx / 64;
		let bit = idx % 64;
		self.bits[byte] |= 1 << bit;
	}
	#[inline(always)]
	fn reset_at_index(&mut self, idx: usize) {
		let byte = idx / 64;
		let bit = idx % 64;
		self.bits[byte] &= !(1 << bit);
	}
	#[inline(always)]
	fn toggle_at_index(&mut self, idx: usize) {
		let byte = idx / 64;
		let bit = idx % 64;
		self.bits[byte] ^= 1 << bit;
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
		let mut a = self.bits.clone();
		for b in a.iter_mut() {
			*b = !(*b);
		}
		Self{ w: self.w, h: self.h, col_major: self.col_major, bits: a }
	}
	#[inline]
	fn lsb(&self) -> u32 {
		self.bits[0].trailing_zeros()
	}
	fn pop_lsb(&mut self) -> u32 {
		for (word_index, word) in self.bits.iter_mut().enumerate() {
			if *word != 0 {
				let lsb = word.trailing_zeros();
				*word &= *word - 1;
				return (word_index as u32) * 64 + lsb;
			}
		}
		u32::MAX
	}

	fn pext(&self, mask: &Self) -> Self::Storage {
		let mut out = Vec::new();
		let mut current: u64 = 0;
		let mut bitpos = 0;

		for (a, m) in self.bits.iter().zip(mask.bits.iter()) {
			let mut mm = *m;
			while mm != 0 {
				let lsb = mm.trailing_zeros();
				let bit = (a >> lsb) & 1;

				current |= bit << bitpos;
				bitpos += 1;

				if bitpos == 64 {
					out.push(current);
					current = 0;
					bitpos = 0;
				}

				mm &= mm - 1;
			}
		}

		if bitpos > 0 {
			out.push(current);
		}

		out.into_boxed_slice()
	}

	fn pdep(&self, compressed: Self::Storage) -> Self {
		let mut out = vec![0u64; self.bits.len()];

		let mut src_word_index = 0;
		let mut src_bit_index = 0;

		for (out_word, mask_word) in out.iter_mut().zip(self.bits.iter()) {
			let mut mm = *mask_word;

			while mm != 0 {
				let dst_bit = mm.trailing_zeros();

				let src_word = compressed[src_word_index];
				let src_bit = (src_word >> src_bit_index) & 1;

				*out_word |= src_bit << dst_bit;

				src_bit_index += 1;
				if src_bit_index == 64 {
					src_bit_index = 0;
					src_word_index += 1;
				}

				mm &= mm - 1;
			}
		}

		Self::new(self.w, self.h, self.col_major, out.into_boxed_slice())
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
		//clear
		*self &= !mask;
		// insert
		*self |= new_row;
	}

	#[inline(always)]
	fn insert_col(&mut self, x: u8, col_bits: Self::Storage) {
		let mask = self.col_mask_with_same_shape(x);

		let new_col = mask.pdep(col_bits);
		//clear
		*self &= !mask;
		// insert
		*self |= new_col;
	}

}
impl std::ops::BitAnd for LargeBitboard {
	type Output = Self;

	#[inline(always)]
	fn bitand(mut self, rhs: Self) -> Self {
		self &= rhs;
		self
	}
}

impl std::ops::BitOr for LargeBitboard {
	type Output = Self;

	#[inline(always)]
	fn bitor(mut self, rhs: Self) -> Self {
		self |= rhs;
		self
	}
}

impl std::ops::BitXor for LargeBitboard {
	type Output = Self;

	#[inline(always)]
	fn bitxor(mut self, rhs: Self) -> Self {
		self ^= rhs;
		self
	}
}


impl std::ops::BitAndAssign<&Self> for LargeBitboard {
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: &Self) {
		for (a, b) in self.bits.iter_mut().zip(rhs.bits.iter()) {
			*a &= *b;
		}
	}
}

impl std::ops::BitOrAssign<&Self> for LargeBitboard {
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: &Self) {
		for (a, b) in self.bits.iter_mut().zip(rhs.bits.iter()) {
			*a |= *b;
		}
	}
}

impl std::ops::BitXorAssign<&Self> for LargeBitboard {
	#[inline(always)]
	fn bitxor_assign(&mut self, rhs: &Self) {
		for (a, b) in self.bits.iter_mut().zip(rhs.bits.iter()) {
			*a ^= *b;
		}
	}
}
impl std::ops::BitAndAssign for LargeBitboard {
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: Self) {
		*self &= &rhs;
	}
}

impl std::ops::BitOrAssign for LargeBitboard {
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: Self) {
		*self |= &rhs;
	}
}

impl std::ops::BitXorAssign for LargeBitboard {
	#[inline(always)]
	fn bitxor_assign(&mut self, rhs: Self) {
		*self ^= &rhs;
	}
}

impl std::ops::Not for LargeBitboard {
	type Output = Self;

	#[inline(always)]
	fn not(mut self) -> Self {
		for a in self.bits.iter_mut() {
			*a = !*a;
		}
		self
	}

}

impl std::ops::Shl<usize> for LargeBitboard {
	type Output = Self;
	fn shl(mut self, rhs: usize) -> Self {
		self <<= rhs;
		self
	}
}

impl std::ops::Shr<usize> for LargeBitboard {
	type Output = Self;
	fn shr(mut self, rhs: usize) -> Self {
		self >>= rhs;
		self
	}
}

impl std::ops::ShlAssign<usize> for LargeBitboard {
	#[inline(always)]
	fn shl_assign(&mut self, rhs: usize) {
		let word_shift = rhs / 64;
		let bit_shift  = rhs % 64;

		let len = self.bits.len();

		if word_shift >= len {
			for w in self.bits.iter_mut() {
				*w = 0;
			}
			return;
		}

		if word_shift > 0 {
			for i in (word_shift..len).rev() {
				self.bits[i] = self.bits[i - word_shift];
			}
			for i in 0..word_shift {
				self.bits[i] = 0;
			}
		}

		if bit_shift > 0 {
			for i in (1..len).rev() {
				self.bits[i] = (self.bits[i] << bit_shift)
					| (self.bits[i - 1] >> (64 - bit_shift));
			}
			self.bits[0] <<= bit_shift;
		}
	}
}

impl std::ops::ShrAssign<usize> for LargeBitboard {
	#[inline(always)]
	fn shr_assign(&mut self, rhs: usize) {
		let word_shift = rhs / 64;
		let bit_shift  = rhs % 64;

		let len = self.bits.len();

		if word_shift >= len {
			for w in self.bits.iter_mut() {
				*w = 0;
			}
			return;
		}

		if word_shift > 0 {
			for i in 0..len - word_shift {
				self.bits[i] = self.bits[i + word_shift];
			}
			for i in len - word_shift..len {
				self.bits[i] = 0;
			}
		}

		if bit_shift > 0 {
			for i in 0..len - 1 {
				self.bits[i] = (self.bits[i] >> bit_shift)
					| (self.bits[i + 1] << (64 - bit_shift));
			}
			self.bits[len - 1] >>= bit_shift;
		}
	}
}
impl std::ops::Shl<u8> for LargeBitboard {
	type Output = Self;

	#[inline(always)]
	fn shl(mut self, rhs: u8) -> Self {
		self <<= rhs as usize;
		self
	}
}

impl std::ops::Shr<u8> for LargeBitboard {
	type Output = Self;

	#[inline(always)]
	fn shr(mut self, rhs: u8) -> Self {
		self >>= rhs as usize;
		self
	}
}
impl std::ops::ShlAssign<u8> for LargeBitboard {
	#[inline(always)]
	fn shl_assign(&mut self, rhs: u8) {
		*self <<= rhs as usize;
	}
}

impl std::ops::ShrAssign<u8> for LargeBitboard {
	#[inline(always)]
	fn shr_assign(&mut self, rhs: u8) {
		*self >>= rhs as usize;
	}
}


impl std::ops::SubAssign<usize> for LargeBitboard {
	#[inline(always)]
	fn sub_assign(&mut self, rhs: usize) {
		let mut carry = rhs as u64;

		for word in self.bits.iter_mut() {
			let (new_word, overflow) = word.overflowing_sub(carry);
			*word = new_word;
			carry = if overflow { 1 } else { 0 };

			if carry == 0 {
				break;
			}
		}
	}
}
impl std::ops::Sub<usize> for LargeBitboard {
	type Output = Self;

	fn sub(mut self, rhs: usize) -> Self::Output {
		self -= rhs;
		self
	}
}


impl std::fmt::Display for LargeBitboard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		fmt_bitboard_display(self, f)
	}
}

impl std::fmt::Debug for LargeBitboard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		fmt_bitboard_debug(self, self.bits.len() * 64, f)
	}
}


#[cfg(test)]
mod tests {
	use crate::runtime::RuntimeBitboard;

	use super::LargeBitboard;

	#[test]
	fn test() {
		let bitboard=LargeBitboard::empty(19,19,false);
		println!("{}", bitboard);
		println!("{}", bitboard.east_border_with_same_shape());
		println!("{}", bitboard.north_border_with_same_shape());
		println!("{}", bitboard.south_border_with_same_shape());
		println!("{}", bitboard.west_border_with_same_shape());
		println!("{:?}", bitboard.full_with_same_shape());
		println!("{}", bitboard.row_mask_with_same_shape(4));
		println!("{}", bitboard.col_mask_with_same_shape(13));
	}
}