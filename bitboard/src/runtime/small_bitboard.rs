use crate::{Bitboard, fmt_bitboard_debug, fmt_bitboard_display, runtime::RuntimeBitboard};
#[derive(Clone, PartialEq, Eq, Copy)]
pub struct SmallBitboard {
	w: u8,
	h: u8,
	col_major: bool,
	bits: u64,
}
impl SmallBitboard {
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
impl SmallBitboard {
	#[inline(always)]
	pub fn borders_with_same_shape(&self) -> Self {
		Self::west_border(self.w, self.h, self.col_major) | Self::east_border(self.w, self.h, self.col_major) |
		Self::north_border(self.w, self.h, self.col_major) | Self::south_border(self.w, self.h, self.col_major)
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
impl Default for SmallBitboard {
	#[inline(always)]
	fn default() -> Self {
		Self { w: 8, h: 8, col_major: false, bits: 0 }
	}
}
impl RuntimeBitboard for SmallBitboard {
	#[inline(always)]
	fn new(w: u8, h: u8, col_major: bool, bits: u64) -> Self {
		assert!(w*h<=64);
		Self { w, h, col_major, bits }
	}
	#[inline(always)]
	fn borders(w: u8, h: u8, col_major: bool) -> Self {
		Self::west_border(w, h, col_major) | Self::east_border(w, h, col_major) |
		Self::north_border(w, h, col_major) | Self::south_border(w, h, col_major)
	}
	#[inline(always)]
	fn west_border(w: u8, h: u8, col_major: bool) -> Self {
		let mut bits = 0u64;
		let mut v = 1u64;
		for _ in 0..h {
			bits |= v;
			v <<= w;
		}

		Self::new(w, h, col_major, bits)
	}

	#[inline(always)]
	fn east_border(w: u8, h: u8, col_major: bool) -> Self {
		let mut bits = 0u64;
		let mut v = 1u64 << (w - 1);

		for _ in 0..h {
			bits |= v;
			v <<= w;
		}

		Self::new(w, h, col_major, bits)
	}
	#[inline(always)]
	fn south_border(w: u8, h: u8, col_major: bool) -> Self {
		let bits = if w == 64 {
			u64::MAX
		} else {
			(1u64 << w) - 1
		};

		Self::new(w, h, col_major, bits)
	}

	#[inline(always)]
	fn north_border(w: u8, h: u8, col_major: bool) -> Self {
		let mut bits = if w == 64 {
			u64::MAX
		} else {
			(1u64 << w) - 1
		};

		bits <<= w * (h - 1);

		Self::new(w, h, col_major, bits)
	}
	#[inline(always)]
	fn full(w: u8, h: u8, col_major: bool) -> Self {
		let n = w as u32 * h as u32;

		let bits = if n == 64 {
			u64::MAX
		} else {
			(1u64 << n) - 1
		};

		Self::new(w, h, col_major, bits)
	}
	
	#[inline(always)]
	fn empty(w: u8, h: u8, col_major: bool) -> Self {
		Self::new(w, h, col_major, 0)
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
	
	fn full_with_same_shape(&self) -> Self {
		Self::full(self.w, self.h, self.col_major)
	}

	fn empty_with_same_shape(&self) -> Self {
		Self::empty(self.w, self.h, self.col_major)
	}

	fn west_border_with_same_shape(&self) -> Self {
		Self::west_border(self.w, self.h, self.col_major)
	}

	fn east_border_with_same_shape(&self) -> Self {
		Self::east_border(self.w, self.h, self.col_major)
	}

	fn north_border_with_same_shape(&self) -> Self {
		Self::north_border(self.w, self.h, self.col_major)
	}

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
	
	fn row_mask(w: u8, h: u8, col_major: bool, y: u8) -> Self {
		if col_major {
			Self::west_border(w,h,col_major) << y
		} else {
			Self{ w, h, col_major, bits: ((1u64 << w) - 1) << (y * w) }
		}
	}
	fn col_mask(w: u8, h: u8, col_major: bool, x: u8) -> Self {
		if col_major {
			Self{ w, h, col_major, bits: ((1u64 << h) - 1) << (x * h) }
		} else {
			Self::west_border(w,h,col_major) << x
		}
	}
	
	fn index_from_coords_with_same_shape(&self, x: u8, y: u8) -> usize {
		Self::index_from_coords(self.w, self.h, self.col_major, x, y)
	}
	
	fn coords_from_index_with_same_shape(&self, i: usize) -> (u8, u8) {
		Self::coords_from_index(self.w, self.h, self.col_major, i)
	}
	
	fn is_in_bounds_with_same_shape(&self, x: u8, y: u8) -> bool {
		Self::is_in_bounds(self.w, self.h, x, y)
	}
	
	fn is_index_in_bounds_with_same_shape(&self, i: usize) -> bool {
		Self::is_index_in_bounds(self.w, self.h, i)
	}
	
}
impl Bitboard for SmallBitboard {
	type Storage = u64;
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
		let idx = self.bits.trailing_zeros();
		self.bits &= self.bits - 1;
		idx
	}
	#[inline]
	fn pext(&self, mask: &Self) -> Self::Storage {
		#[cfg(debug_assertions)]
		eprintln!("pext");
		#[cfg(target_feature = "bmi2")]
		unsafe {
			std::arch::x86_64::_pext_u64(self.bits, mask.bits)
		}
		#[cfg(not(target_feature = "bmi2"))]
		{
			#[cfg(debug_assertions)]
			eprintln!("pext fallback");
			let mut res: u64 = 0;
			let mut bit: u64 = 1;
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
			//#[cfg(debug_assertions)]
			//eprintln!("pdep_bmi2");
			Self{w:self.w, h:self.h, col_major: self.col_major, bits:std::arch::x86_64::_pdep_u64(compressed, self.bits)}
		}
		#[cfg(not(target_feature = "bmi2"))]
		{
			#[cfg(debug_assertions)]
			eprintln!("pdep fallback");
			let mut res: u64 = 0;
			let mut bit: u64 = 1;
			let mut m = self.bits;
			let mut v = compressed;

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
			Self{w:self.w, h:self.h, col_major: self.col_major, bits: res}
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
impl std::ops::BitAnd for SmallBitboard {
	type Output = Self;

	#[inline(always)]
	fn bitand(mut self, rhs: Self) -> Self {
		self.bits &= rhs.bits;
		self
	}
}

impl std::ops::BitOr for SmallBitboard {
	type Output = Self;

	#[inline(always)]
	fn bitor(mut self, rhs: Self) -> Self {
		self.bits |= rhs.bits;
		self
	}
}

impl std::ops::BitXor for SmallBitboard {
	type Output = Self;

	#[inline(always)]
	fn bitxor(mut self, rhs: Self) -> Self {
		self.bits ^= rhs.bits;
		self
	}
}
impl std::ops::BitAnd<&Self> for SmallBitboard {
	type Output = Self;

	#[inline(always)]
	fn bitand(mut self, rhs: &Self) -> Self {
		self.bits &= rhs.bits;
		self
	}
}
impl std::ops::BitOr<&Self> for SmallBitboard {
	type Output = Self;

	#[inline(always)]
	fn bitor(mut self, rhs: &Self) -> Self {
		self.bits |= rhs.bits;
		self
	}
}
impl std::ops::BitXor<&Self> for SmallBitboard {
	type Output = Self;

	#[inline(always)]
	fn bitxor(mut self, rhs: &Self) -> Self {
		self.bits ^= rhs.bits;
		self
	}
}

impl std::ops::BitAndAssign for SmallBitboard {
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: Self) {
		self.bits &= rhs.bits;
	}
}

impl std::ops::BitOrAssign for SmallBitboard {
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: Self) {
		self.bits |= rhs.bits;
	}
}

impl std::ops::BitXorAssign for SmallBitboard {
	#[inline(always)]
	fn bitxor_assign(&mut self, rhs: Self) {
		self.bits ^= rhs.bits;
	}
}
impl std::ops::Not for SmallBitboard {
	type Output = Self;

	#[inline(always)]
	fn not(mut self) -> Self {
		self.bits = !self.bits;
		self
	}
}

impl std::ops::Shl<usize> for SmallBitboard {
	type Output = Self;

	#[inline(always)]
	fn shl(mut self, rhs: usize) -> Self {
		self.bits <<= rhs;
		self
	}
}

impl std::ops::Shr<usize> for SmallBitboard {
	type Output = Self;

	#[inline(always)]
	fn shr(mut self, rhs: usize) -> Self {
		self.bits >>= rhs;
		self
	}
}

impl std::ops::ShlAssign<usize> for SmallBitboard {
	#[inline(always)]
	fn shl_assign(&mut self, rhs: usize) {
		self.bits <<= rhs;
	}
}
impl std::ops::ShrAssign<usize> for SmallBitboard {
	#[inline(always)]
	fn shr_assign(&mut self, rhs: usize) {
		self.bits >>= rhs;
	}
}
impl std::ops::Shl<u8> for SmallBitboard {
	type Output = Self;

	#[inline(always)]
	fn shl(mut self, rhs: u8) -> Self {
		self.bits <<= rhs;
		self
	}
}

impl std::ops::Shr<u8> for SmallBitboard {
	type Output = Self;

	#[inline(always)]
	fn shr(mut self, rhs: u8) -> Self {
		self.bits >>= rhs;
		self
	}
}

impl std::fmt::Display for SmallBitboard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		fmt_bitboard_display(self, f)
	}
}

impl std::fmt::Debug for SmallBitboard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		fmt_bitboard_debug(self, std::mem::size_of_val(self.storage()) * 8, f)
	}
}
impl SmallBitboard {
	pub fn generate_sliding_moves(offsets: &[(i8, i8)], w: u8, h: u8, col_major: bool)
		-> Vec<Self>
	{
		let mut attacks = Vec::with_capacity((w as usize) * (h as usize));

		for y in 0..h {
			for x in 0..w {
				let mut bb = SmallBitboard::empty(w, h, col_major);

				for &(dx, dy) in offsets {
					let mut nx = x as i8 + dx;
					let mut ny = y as i8 + dy;

					while nx >= 0 && ny >= 0 &&
						  (nx as u8) < w && (ny as u8) < h
					{
						bb.set(nx as u8, ny as u8);

						nx += dx;
						ny += dy;
					}
				}

				attacks.push(bb);
			}
		}

		attacks
	}
	pub fn generate_sliding_moves2(
		offsets: &[(i8, i8)],
		w: u8,
		h: u8,
		col_major: bool,
	) -> Vec<Self> {
		let mut moves = Vec::with_capacity((w as usize) * (h as usize));

		for y in 0..h {
			for x in 0..w {
				let mut bb = SmallBitboard::empty(w, h, col_major);

				for &(dx, dy) in offsets {
					// Horizontal sliding
					if dy == 0 {
						let mut row = bb.extract_row(y);
						let mut cx = x as i8 + dx;

						while cx >= 0 && (cx as u8) < w {
							row |= 1 << (cx as u8);
							cx += dx;
						}

						bb.insert_row(y, row);
						continue;
					}

					// Vertical sliding
					if dx == 0 {
						let mut col = bb.extract_col(x);
						let mut cy = y as i8 + dy;

						while cy >= 0 && (cy as u8) < h {
							col |= 1 << (cy as u8);
							cy += dy;
						}

						bb.insert_col(x, col);
						continue;
					}

					// Diagonals → fallback naïf (on optimisera après)
					let mut nx = x as i8 + dx;
					let mut ny = y as i8 + dy;

					while nx >= 0 && ny >= 0 &&
						(nx as u8) < w && (ny as u8) < h
					{
						bb.set(nx as u8, ny as u8);
						nx += dx;
						ny += dy;
					}
				}

				moves.push(bb);
			}
		}

		moves
	}

}
impl SmallBitboard {
	pub fn generate_jump_moves(offsets: &[(i8, i8)], w: u8, h: u8, col_major: bool)
		-> Vec<Self>
	{
		let mut moves = Vec::with_capacity((w as usize) * (h as usize));

		for y in 0..h {
			for x in 0..w {
				let mut bb = SmallBitboard::empty(w, h, col_major);

				for &(dx, dy) in offsets {
					let nx = x as i8 + dx;
					let ny = y as i8 + dy;

					if nx >= 0 && ny >= 0 &&
					   (nx as u8) < w && (ny as u8) < h
					{
						bb.set(nx as u8, ny as u8);
					}
				}

				moves.push(bb);
			}
		}

		moves
	}
	/*
	pub fn generate_sliding_table<S: Sliding>(&self) -> SlidingTable<S::BB>
	where
		S: Sliding,
		S::BB: RuntimeBitboard + Clone
	{
		let board_size = self.height() as usize * self.width() as usize;
		let mut table = SlidingTable {
			mask: vec![S::BB::empty(self.width(), self.height(), self.col_major()); board_size],
			attacks: vec![Vec::new(); board_size],
		};
		let mut sq=0;
		while sq < board_size {
			let mask = S::mask(sq as u8);
			let subsets = mask.all_subsets();
			let mut atk_table = vec![S::BB::empty(self.width(), self.height(), self.col_major()); 1 << mask.count()];

			for subset in subsets {
				let idx = subset.pext(&mask);
				atk_table[idx.to_usize()] = S::attacks(sq as u8, &subset);
			}

			table.mask[sq] = mask;
			table.attacks[sq] = atk_table;
			sq+=1;
		}

		table
	}
	*/

}

#[cfg(test)]
mod tests {
	use crate::{BitIter, runtime::RuntimeBitboard};

	use super::SmallBitboard;

	#[test]
	fn test() {
		let mut bitboard=SmallBitboard::new(7,8,false,0);
		println!("{}", bitboard);
		println!("{}", bitboard.east_border_with_same_shape());
		println!("{}", bitboard.north_border_with_same_shape());
		println!("{}", bitboard.south_border_with_same_shape());
		println!("{}", bitboard.west_border_with_same_shape());
		println!("{:?}", bitboard.full_with_same_shape());
		println!("{}", bitboard.row_mask_with_same_shape(4));
		println!("{}", bitboard.col_mask_with_same_shape(3));
		bitboard|=SmallBitboard::new(7,8,false,0x0D);
		for i in bitboard.iter_bits() {
			println!("{}",i);
		}
	}
	#[test]
	fn test_moves() {
		pub const KNIGHT_OFFSETS: &[(i8,i8)] = &[
			(1,2),(2,1),(2,-1),(1,-2),
			(-1,-2),(-2,-1),(-2,1),(-1,2),
		];
		let moves= SmallBitboard::generate_jump_moves(KNIGHT_OFFSETS,8,8, false);
		println!("{}", moves[27]);
		let bishop_dirs = vec![(1,1), (1,-1), (-1,1), (-1,-1)];
		let moves = SmallBitboard::generate_sliding_moves(&bishop_dirs, 8, 8, false);
		println!("{}", moves[27]);
		let rook_dirs = vec![(1,0), (0,-1), (-1,0), (0,1)];
		let moves = SmallBitboard::generate_sliding_moves(&rook_dirs, 8, 8, false);
		println!("{}", moves[27]);
		let moves = SmallBitboard::generate_sliding_moves(&KNIGHT_OFFSETS, 8, 8, false);
		println!("{}", moves[27]);
		let moves= SmallBitboard::generate_jump_moves(&[(1,1),(-1,1)],8,8, false);
		println!("{}", moves[27]);
		let moves= SmallBitboard::generate_jump_moves(&[(1,-1),(-1,-1)],8,8, false);
		println!("{}", moves[27]);
		
	}
}

