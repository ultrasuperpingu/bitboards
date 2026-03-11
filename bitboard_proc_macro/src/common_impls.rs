use quote::quote;

pub(crate) fn common_impl(ident: &syn::Ident, width_u8:u8, height_u8:u8, col_major: bool) -> proc_macro2::TokenStream {
	quote! {
		impl #ident {
			/// Width of the Bitboard
			pub const WIDTH: u8 = #width_u8;
			/// Height of the Bitboard
			pub const HEIGHT: u8 = #height_u8;
			/// Total number of squares in the bitboard
			pub const NB_SQUARES: usize = Self::WIDTH as usize * Self::HEIGHT as usize;
			/// Whether the square indexes are in column-major order
			pub const COL_MAJOR: bool = #col_major;
			/// Offset to add/subtract to an index to move to the next column
			pub const H_OFFSET: usize = if Self::COL_MAJOR { Self::HEIGHT as usize } else { 1 };
			/// Offset to add/subtract to an index to move to the next row
			pub const V_OFFSET: usize = if Self::COL_MAJOR { 1 } else { Self::WIDTH as usize };
			/// Offset to add/subtract to an index to move to the top-right diagonal square
			pub const DIAG_INC_OFFSET: u8 = Self::WIDTH + 1;
			/// Offset to add/subtract to an index to move to the bottom-left diagonal square
			pub const DIAG_DEC_OFFSET: u8 = Self::WIDTH - 1;

			#[inline]
			pub const fn new() -> Self {
				Self::empty()
			}
			#[inline]
			pub const fn empty() -> Self {
				Self::EMPTY
			}
			/// Returns `true` if this bitboard intersects with another (i.e., they share at least one set bit).
			#[inline(always)]
			pub const fn intersects(&self, other: &Self) -> bool {
				let self_cp = Self(self.0);
				self_cp.and_const(&Self(other.0)).any()
			}
			/// Returns `(x, y)` coordinates corresponding to a linear index `i`.
			#[inline]
			pub const fn coords_from_index(i: usize) -> (u8, u8) {
				if Self::COL_MAJOR {
					((i / Self::HEIGHT as usize) as u8, (i % Self::HEIGHT as usize) as u8)
				} else {
					((i % Self::WIDTH as usize) as u8, (i / Self::WIDTH as usize) as u8)
				}
			}
			/// Returns the linear index corresponding to coordinates `(x, y)`.
			#[inline]
			pub const fn index_from_coords(x: u8, y: u8) -> usize {
				if Self::COL_MAJOR {
					x as usize * Self::HEIGHT as usize + y as usize
				} else {
					y as usize * Self::WIDTH as usize + x as usize
				}
			}
			
			/// Generates a table of sliding attack bitboards given movement `offsets`.
			/// Each entry corresponds to attacks from a square in the bitboard.
			pub fn generate_sliding_attacks_table(offsets: &[(i8, i8)]) -> [#ident; Self::NB_SQUARES] 
			{
				let mut attacks = [Self::EMPTY;Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					let (x,y) = Self::coords_from_index(i);
					let mut bb = Self::EMPTY;

					for &(dx, dy) in offsets {
						let mut nx = x as i8 + dx;
						let mut ny = y as i8 + dy;

						while nx >= 0 && ny >= 0 &&
							(nx as u8) < Self::WIDTH as u8 && (ny as u8) < Self::HEIGHT as u8
						{
							bb.set_at_index(i);

							nx += dx;
							ny += dy;
						}
					}

					attacks[i] = bb;
					i += 1;
				}

				attacks
			}
			
			/// Generates a table of jump attack bitboards given movement `offsets`.
			/// Each entry corresponds to single-step jumps from a square.
			pub const fn generate_jump_attacks_table(offsets: &[(i8, i8)]) -> [#ident; Self::NB_SQUARES] {
				let mut out = [Self::EMPTY;Self::NB_SQUARES];

				let mut i = 0;
				while i < Self::NB_SQUARES {
					let (x,y) = Self::coords_from_index(i);

					let mut bb = Self::EMPTY;

					let mut j = 0;
					while j < offsets.len() {
						let dx = offsets[j].0;
						let dy = offsets[j].1;

						let nx = x as i8 + dx;
						let ny = y as i8 + dy;

						if nx >= 0 && ny >= 0 &&
						(nx as u8) < Self::WIDTH as u8 &&
						(ny as u8) < Self::HEIGHT as u8
						{
							let dest_index = Self::index_from_coords(nx as u8, ny as u8);
							bb.set_at_index(dest_index);
						}

						j += 1;
					}

					out[i] = bb;
					i += 1;
				}

				out
			}

			/// Generates the full ray-between mask table.
			pub const fn generate_ray_between_table() -> [[Self; Self::NB_SQUARES]; Self::NB_SQUARES] {
				let mut table = [const { [Self::EMPTY; Self::NB_SQUARES] }; Self::NB_SQUARES];
				let mut from = 0;

				while from < Self::NB_SQUARES {
					let mut to = 0;
					while to < Self::NB_SQUARES {
						table[from][to] = Self::compute_ray_between_mask(from, to);
						to += 1;
					}
					from += 1;
				}

				table
			}
			/// Computes the bitboard mask of all squares strictly between `from` and `to` on the same line.
			const fn compute_ray_between_mask(from: usize, to: usize) -> Self {
				let (fx, fy) = Self::coords_from_index(from);
				let (tx, ty) = Self::coords_from_index(to);

				let same_file = fx == tx;
				let same_rank = fy == ty;
				let same_diag = (fx as i8 - fy as i8) == (tx as i8 - ty as i8);
				let same_anti = (fx as i8 + fy as i8) == (tx as i8 + ty as i8);

				if !(same_file || same_rank || same_diag || same_anti) {
					return Self::empty();
				}

				let dx = (tx as i8 - fx as i8).signum();
				let dy = (ty as i8 - fy as i8).signum();

				let mut x = fx as i8 + dx;
				let mut y = fy as i8 + dy;

				let mut bb = Self::empty();

				while x != tx as i8 || y != ty as i8 {
					//bb = Self::from_storage(
					//	bb.storage() |
					//	Self::from_coords(x as u8, y as u8).storage()
					//);
					bb = bb.or_const(&Self::from_coords(x as u8, y as u8));
					x += dx;
					y += dy;
				}

				bb
			}
			#[inline]
			pub const fn has_n_aligned(&self, n: u8) -> bool {
				if n == 0 { return true; }
				if n == 1 { return self.any(); }

				self.has_n_aligned_horizontal(n) ||
					self.has_n_aligned_vertical(n) ||
					self.has_n_aligned_diag_dec(n) ||
					self.has_n_aligned_diag_inc(n)
			}
			#[inline]
			pub const fn has_n_aligned_horizontal(&self, n: u8) -> bool {
				if n == 0 { return true; }
				if n == 1 { return self.any(); }

				let mut temp = Self(self.0);
				let mut i=1;
				while i < n {
					if !Self::COL_MAJOR {
						//let west_mask = !Self::WEST_BORDER.storage();
						//temp&=(temp&west_mask)>>Self::H_OFFSET;
						let mask = Self::WEST_BORDER.not_const();
						let shifted = temp.and_const(&mask).shr_const(Self::H_OFFSET as usize);
						temp = temp.and_const(&shifted);
					} else {
						//temp &= temp >> Self::H_OFFSET;
						let shifted = temp.shr_const(Self::H_OFFSET as usize);
						temp = temp.and_const(&shifted);
					}
					i += 1;
				}
				temp.any()
			}
			#[inline]
			pub const fn has_n_aligned_vertical(&self, n: u8) -> bool {
				if n == 0 { return true; }
				if n == 1 { return self.any(); }

				let mut temp = Self(self.0);
				let mut i=1;
				while i < n {
					if Self::COL_MAJOR {
						//let south_mask = !Self::SOUTH_BORDER.storage();
						//temp&=(temp&south_mask)>>Self::V_OFFSET;
						let mask = Self::SOUTH_BORDER.not_const();
						let shifted = temp.and_const(&mask).shr_const(Self::V_OFFSET as usize);
						temp = temp.and_const(&shifted);
					} else {
						//temp &= temp >> Self::V_OFFSET;
						let shifted = temp.shr_const(Self::V_OFFSET as usize);
						temp = temp.and_const(&shifted);
					}
					i += 1;
				}
				temp.any()
			}
			#[inline]
			pub const fn has_n_aligned_diag_dec(&self, n: u8) -> bool {
				if n <= 1 { return n == 1 && self.any() || n == 0; }
				let mut temp = Self(self.0);
				let mut i = 1;
				let mask = Self::EAST_BORDER.not_const(); 

				while i < n {
					// temp &= (temp & mask) >> offset
					let can_slide = temp.and_const(&mask);
					let shifted = can_slide.shr_const(Self::DIAG_DEC_OFFSET as usize);
					temp = temp.and_const(&shifted);

					i += 1;
				}
				temp.any()
			}

			#[inline]
			pub const fn has_n_aligned_diag_inc(&self, n: u8) -> bool {
				if n <= 1 { return n == 1 && self.any() || n == 0; }
				let mut temp = Self(self.0);
				let mut i = 1;
				let mask = Self::WEST_BORDER.not_const();
				while i < n {
					// temp = temp & ((temp & mask) >> offset)
					let shifted = temp.and_const(&mask).shr_const(Self::DIAG_INC_OFFSET as usize);
					temp = temp.and_const(&shifted);
					i += 1;
				}
				temp.any()
			}
		}
	}
}
