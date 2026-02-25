pub mod runtime;
extern crate self as bitboard;
#[macro_export]
macro_rules! bitboard_table {
	($table:ident, $access_fn:ident, $mask_fn:ident, $ty:ty, $gen:expr) => {
		impl $ty {
			pub const $table: [$ty; <$ty>::NB_SQUARES] = $gen;
			#[inline(always)]
			pub const fn $mask_fn(index: usize) -> $ty {
				Self::$table[index]
			}
			#[inline(always)]
			pub const fn $access_fn(&self, index: usize) -> $ty {
				Self::from_storage(self.storage() & Self::$table[index].storage())
			}
		}
	};
}


pub trait BitStorage:
	Copy
	+ From<u8>
	+ std::ops::Add<Output = Self>
	+ std::ops::Sub<Output = Self>
	+ std::ops::Mul<Output = Self>
	+ std::ops::Div<Output = Self>
	+ std::ops::Rem<Output = Self>
	+ std::ops::BitAnd<Output = Self>
	+ std::ops::BitOr<Output = Self>
	+ std::ops::BitXor<Output = Self>
	+ std::ops::Not<Output = Self>
	+ std::ops::Shl<usize, Output = Self>
	+ std::ops::Shr<usize, Output = Self>
	+ std::ops::ShlAssign<usize>
	+ std::ops::ShrAssign<usize>
{
}
impl BitStorage for u8 {}
impl BitStorage for u16 {}
impl BitStorage for u32 {}
impl BitStorage for u64 {}
impl BitStorage for u128 {}pub trait IntegerStorage: BitStorage + Copy {
	fn to_u64(self) -> u64;
	fn to_usize(self) -> usize;
	fn from_u64(val: u64) -> Self;
	fn from_usize(val: usize) -> Self;
	fn from_u128(val: u128) -> Self;
}

impl IntegerStorage for u8 {
	fn to_u64(self) -> u64 { self as u64 }
	fn to_usize(self) -> usize { self as usize }
	fn from_u64(val: u64) -> Self { val as u8 }
	fn from_usize(val: usize) -> Self { val as u8 }
	fn from_u128(val: u128) -> Self { val as u8 }
}

impl IntegerStorage for u16 {
	fn to_u64(self) -> u64 { self as u64 }
	fn to_usize(self) -> usize { self as usize }
	fn from_u64(val: u64) -> Self { val as u16 }
	fn from_usize(val: usize) -> Self { val as u16 }
	fn from_u128(val: u128) -> Self { val as u16 }
}

impl IntegerStorage for u32 {
	fn to_u64(self) -> u64 { self as u64 }
	fn to_usize(self) -> usize { self as usize }
	fn from_u64(val: u64) -> Self { val as u32 }
	fn from_usize(val: usize) -> Self { val as u32 }
	fn from_u128(val: u128) -> Self { val as u32 }
}

impl IntegerStorage for u64 {
	fn to_u64(self) -> u64 { self }
	fn to_usize(self) -> usize { self as usize }
	fn from_u64(val: u64) -> Self { val }
	fn from_usize(val: usize) -> Self { val as u64 }
	fn from_u128(val: u128) -> Self { val as u64 }
}

impl IntegerStorage for u128 {
	fn to_u64(self) -> u64 { self as u64 }
	fn to_usize(self) -> usize { self as usize }
	fn from_u64(val: u64) -> Self { val as u128 }
	fn from_usize(val: usize) -> Self { val as u128 }
	fn from_u128(val: u128) -> Self { val }
}

/// A generic 2D bitboard abstraction.
/// 
/// This trait defines the core operations for representing and manipulating
/// a 2D grid of bits (cells), up to 255x255, typically used for board games, masks,
/// spatial indexing, or fast bit‑wise algorithms.
/// 
/// Implementations may use any internal storage type (e.g. `u64`, `u128`,
/// arrays of integers, SIMD vectors…), as long as they respect the API.
/// 
/// Coordinates (x, y) are 0‑based.
/// The origin (0,0) is the bottom‑left cell.
/// x increases to the right (east).
/// y increases upward (north).
pub trait Bitboard : Clone + PartialEq
	+ std::ops::BitAnd<Output = Self>
	+ std::ops::BitOr<Output = Self>
	+ std::ops::BitXor<Output = Self>
	+ std::ops::BitAndAssign
	+ std::ops::BitOrAssign
	+ std::ops::BitXorAssign
	+ std::ops::Not<Output = Self>
	+ std::ops::Shl<usize, Output = Self>
	+ std::ops::Shr<usize, Output = Self>
	+ std::ops::ShlAssign<usize>
	+ std::ops::ShrAssign<usize>
{
	/// Underlying storage type (e.g. `u64`, `u128`, `[u64; 2]`, …).
	type Storage;

	/// Is Bitboard empty
	fn is_empty(&self) -> bool;
	/// Is any bit set to 1
	#[inline]
	fn any(&self) -> bool { !self.is_empty() }
	/// Number of bits set to 1
	fn count(&self) -> u32;
	/// Does the Bitboard intersect the other
	fn intersects(&self, other: &Self) -> bool;
	/// Width of the 2D grid in cells.
	fn width(&self) -> u8;
	/// Height of the 2D grid in cells.
	fn height(&self) -> u8;
	/// Returns `true` if the bitboard is stored in column-major order,
	/// `false` if stored in row-major order.
	/// This affects how `(x, y)` is mapped to a bit index. 
	fn col_major(&self) -> bool;

	/// Returns a reference to the underlying storage.
	fn storage(&self) -> &Self::Storage;
	/// Returns a mutable reference to the underlying storage.
	fn storage_mut(&mut self) -> &mut Self::Storage;
	/// Returns the bit at the given linear index.
	fn get_at_index(&self, index:usize) -> bool;
	/// Sets the bit value at the given linear index.
	fn set_value_at_index(&mut self, index:usize, val: bool);
	/// Sets the bit to 1 at the given linear index.
	fn set_at_index(&mut self, idx: usize);
	/// Sets the bit to 0 at the given linear index.
	fn reset_at_index(&mut self, idx: usize);
	/// Toggle the bit value at the given linear index.
	fn toggle_at_index(&mut self, idx: usize);

	/// Returns the bit at coordinates `(x, y)`.
	fn get(&self, x: u8, y: u8) -> bool;
	/// Sets the bit value at coordinates `(x, y)`.
	fn set_value(&mut self, x: u8, y: u8, val: bool);
	/// Sets the bit to 1 at coordinates `(x, y)`.
	fn set(&mut self, x: u8, y: u8);
	/// Sets the bit to 0 at coordinates `(x, y)`.
	fn reset(&mut self, x: u8, y: u8);

	/// Get flipped bitboard (ie: !self & full)
	fn flipped(&self) -> Self;
	// Get first (lsb) one index
	fn lsb(&self) -> u32;
	/// Set first (lsb) one to zero and returns its index
	fn pop_lsb(&mut self) -> u32;
	/// Parallel bit extract (PEXT).
	///
	/// Extracts bits from `self` according to `mask`, packing them densely. 
	/// Implementations may use BMI2 (`_pext_u64`) when available.
	fn pext(&self, mask: &Self) -> Self::Storage;
	 /// Parallel bit deposit (PDEP). 
	 ///
	 /// Deposits bits from `compressed` into the positions specified by `self`.
	 /// Implementations may use BMI2 (`_pdep_u64`) when available.
	fn pdep(&self, compressed: Self::Storage) -> Self;

	/// Extracts the bits of row `y` as a compact bitfield.
	fn extract_row(&self, y: u8) -> Self::Storage;
	/// Extracts the bits of column `x` as a compact bitfield.
	fn extract_col(&self, x: u8) -> Self::Storage;
	/// Inserts a compact bitfield into row `y`.
	fn insert_row(&mut self,y: u8, row_bits: Self::Storage);
	/// Inserts a compact bitfield into column `x`.
	fn insert_col(&mut self,x: u8, col_bits: Self::Storage);


}

pub trait Pdep<T:Bitboard> {
	fn pdep(&self, mask: &T) -> T;
}
impl<T> Pdep<T> for T::Storage
	where T: Bitboard, T::Storage:Clone
{
	fn pdep(&self, mask: &T) -> T {
		T::pdep(mask, self.clone())
	}
}
pub trait BitIter: Bitboard+Sized
	where <Self as Bitboard>::Storage: BitStorage
{
	fn iter_bits(self) -> BitIterator<Self>;
}
pub struct BitIterator<B: Bitboard>
where <B as Bitboard>::Storage: BitStorage
{
	bb: B,
}

impl<B: Bitboard> Iterator for BitIterator<B>
where <B as Bitboard>::Storage: BitStorage
{
	type Item = u32;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		if self.bb.any() {
			Some(self.bb.pop_lsb())
		} else {
			None
		}
	}
}
impl<B: Bitboard> BitIter for B
where <B as Bitboard>::Storage: BitStorage
{
	#[inline]
	fn iter_bits(self) -> BitIterator<Self> {
		BitIterator { bb: self }
	}
}

//impl #struct_ident {
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


pub fn fmt_bitboard_display<B: Bitboard>(b: &B, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	const MAX_W: u8 = 70;
	const MAX_H: u8 = 70;
	let w = b.width().min(MAX_W);
	let h = b.height().min(MAX_H);

	let line_index_width = ((h - 1).to_string().len()).max(2);

	if b.height() > MAX_H {
		writeln!(f, "{:>width$}   ...", "", width = line_index_width)?;
	}

	for y in (0..h).rev() {
		write!(f, "{:>width$} | ", y, width = line_index_width)?;

		for x in 0..w {
			let c = if b.get(x, y) { '#' } else { '.' };
			write!(f, "{}", c)?;
		}

		if b.width() > MAX_W {
			write!(f, "...")?;
		}

		writeln!(f)?;
	}

	let prefix = " ".repeat(line_index_width + 3);

	write!(f, "{}", prefix)?;
	for x in 0..w {
		if x >= 10 {
			write!(f, "{}", (x / 10) % 10)?;
		} else {
			write!(f, "{}", x)?;
		}
	}
	if b.width() > MAX_W {
		write!(f, "...")?;
	}
	writeln!(f)?;

	if b.width() >= 10 {
		write!(f, "{}", prefix)?;

		for x in 0..w {
			if x < 10 {
				write!(f, " ")?;
			} else {
				write!(f, "{}", x % 10)?;
			}
		}

		if b.width() > MAX_W {
			write!(f, "...")?;
		}

		writeln!(f)?;
	}

	Ok(())
}


pub fn fmt_bitboard_debug<B: Bitboard>(b: &B, storage_bits:usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	let total = b.width() as usize * b.height() as usize;
	//let storage_bits = std::mem::size_of_val(&b.storage()) * 8;

	let mut bits = Vec::new();
	for i in 0..storage_bits {
		bits.push(b.get_at_index(i));
	}

	let valid = &bits[..total];
	let invalid = &bits[total..];

	write!(f, "{}(0b", std::any::type_name::<B>())?;


	if invalid.iter().any(|b| *b) {
		write!(f, "(")?;
		for b in invalid.iter().rev() { // MSB → LSB
			write!(f, "{}", if *b { '1' } else { '0' })?;
		}
		write!(f, ")")?;
	}

	let mut valid_rev = valid.to_vec();
	valid_rev.reverse();

	let group_size = if b.col_major() {
		b.height()
	} else {
		b.width()
	} as usize;

	for chunk in valid_rev.chunks(group_size) {
		if chunk.as_ptr() != valid_rev.as_ptr() {
			write!(f, "_")?;
		}
		for b in chunk {
			write!(f, "{}", if *b { '1' } else { '0' })?;
		}
	}

	write!(f, ")")
}
