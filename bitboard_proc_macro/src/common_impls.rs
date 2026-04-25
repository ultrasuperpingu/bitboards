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
			/// Computes the orthogonal neighbors (N, S, E, W) of the square at `index`.
			#[inline]
			pub const fn compute_neighbors_ortho_mask(index: usize) -> Self {
				let (x, y) = Self::coords_from_index(index);
				let mut bb = Self::EMPTY;

				if x > 0 {
					bb = bb.or_const(&Self::from_index(Self::index_from_coords(x - 1, y)));
				}
				if x + 1 < Self::WIDTH {
					bb = bb.or_const(&Self::from_index(Self::index_from_coords(x + 1, y)));
				}
				if y > 0 {
					bb = bb.or_const(&Self::from_index(Self::index_from_coords(x, y - 1)));
				}
				if y + 1 < Self::HEIGHT {
					bb = bb.or_const(&Self::from_index(Self::index_from_coords(x, y + 1)));
				}

				bb
			}
			/// Generates a table of orthogonal neighbors for all squares.
			pub const fn generate_neighbors_ortho_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self::EMPTY; Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_neighbors_ortho_mask(i);
					i += 1;
				}
				arr
			}
			/// Computes the diagonal neighbors (NW, NE, SW, SE) of the square at `index`.
			#[inline]
			pub const fn compute_neighbors_diag_mask(index: usize) -> Self {
				let (x, y) = Self::coords_from_index(index);
				let mut bb = Self::EMPTY;

				// NW
				if x > 0 && y + 1 < Self::HEIGHT {
					let idx = Self::index_from_coords(x - 1, y + 1);
					bb = bb.or_const(&Self::from_index(idx));
				}

				// NE
				if x + 1 < Self::WIDTH && y + 1 < Self::HEIGHT {
					let idx = Self::index_from_coords(x + 1, y + 1);
					bb = bb.or_const(&Self::from_index(idx));
				}

				// SW
				if x > 0 && y > 0 {
					let idx = Self::index_from_coords(x - 1, y - 1);
					bb = bb.or_const(&Self::from_index(idx));
				}

				// SE
				if x + 1 < Self::WIDTH && y > 0 {
					let idx = Self::index_from_coords(x + 1, y - 1);
					bb = bb.or_const(&Self::from_index(idx));
				}

				bb
			}
			/// Generates a table of diagonal neighbors for all squares.
			pub const fn generate_neighbors_diag_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self::EMPTY; Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_neighbors_diag_mask(i);
					i += 1;
				}
				arr
			}
			/// Computes all 8 neighbors (orthogonal + diagonal) for a given square.
			#[inline]
			pub const fn compute_neighbors_8_mask(index: usize) -> Self {
				let ortho = Self::compute_neighbors_ortho_mask(index);
				let diag  = Self::compute_neighbors_diag_mask(index);
				ortho.or_const(&diag)
			}
			/// Generates a table of all 8 neighbors for all squares.
			pub const fn generate_neighbors_8_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self::EMPTY; Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_neighbors_8_mask(i);
					i += 1;
				}
				arr
			}
			/// Computes a ray from `index` in direction `(dx, dy)` until board edge.
			#[inline]
			const fn compute_ray_mask(index: usize, dx: isize, dy: isize) -> Self {
				let (mut x, mut y) = Self::coords_from_index(index);
				let mut bb = Self::EMPTY;

				loop {
					let nx = x as isize + dx;
					let ny = y as isize + dy;

					if nx < 0 || ny < 0 {
						break;
					}
					if nx >= Self::WIDTH as isize || ny >= Self::HEIGHT as isize {
						break;
					}

					x = nx as u8;
					y = ny as u8;

					let idx = Self::index_from_coords(x, y);
					bb = bb.or_const(&Self::from_index(idx));
				}

				bb
			}
			/// Computes a ray from `index` in north direction until board edge.
			#[inline]
			pub const fn compute_ray_n_mask(index: usize) -> Self {
				Self::compute_ray_mask(index, 0, 1)
			}
			/// Generates a table of north ray for all squares.
			pub const fn generate_ray_n_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self::EMPTY; Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_ray_n_mask(i);
					i += 1;
				}
				arr
			}


			/// Computes a ray from `index` in south direction until board edge.
			#[inline]
			pub const fn compute_ray_s_mask(index: usize) -> Self {
				Self::compute_ray_mask(index, 0, -1)
			}
			/// Generates a table of south ray for all squares.
			pub const fn generate_ray_s_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self::EMPTY; Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_ray_s_mask(i);
					i += 1;
				}
				arr
			}

			/// Computes a ray from `index` in east direction until board edge.
			#[inline]
			pub const fn compute_ray_e_mask(index: usize) -> Self {
				Self::compute_ray_mask(index, 1, 0)
			}
			/// Generates a table of east ray for all squares.
			pub const fn generate_ray_e_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self::EMPTY; Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_ray_e_mask(i);
					i += 1;
				}
				arr
			}

			/// Computes a ray from `index` in west direction until board edge.
			#[inline]
			pub const fn compute_ray_w_mask(index: usize) -> Self {
				Self::compute_ray_mask(index, -1, 0)
			}
			/// Generates a table of west ray for all squares.
			pub const fn generate_ray_w_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self::EMPTY; Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_ray_w_mask(i);
					i += 1;
				}
				arr
			}

			/// Computes a ray from `index` in north-east direction until board edge.
			#[inline]
			pub const fn compute_ray_ne_mask(index: usize) -> Self {
				Self::compute_ray_mask(index, 1, 1)
			}
			/// Generates a table of north-east ray for all squares.
			pub const fn generate_ray_ne_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self::EMPTY; Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_ray_ne_mask(i);
					i += 1;
				}
				arr
			}

			/// Computes a ray from `index` in north-west direction until board edge.
			#[inline]
			pub const fn compute_ray_nw_mask(index: usize) -> Self {
				Self::compute_ray_mask(index, -1, 1)
			}
			/// Generates a table of north-west ray for all squares.
			pub const fn generate_ray_nw_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self::EMPTY; Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_ray_nw_mask(i);
					i += 1;
				}
				arr
			}

			/// Computes a ray from `index` in south-east direction until board edge.
			#[inline]
			pub const fn compute_ray_se_mask(index: usize) -> Self {
				Self::compute_ray_mask(index, 1, -1)
			}
			/// Generates a table of south-east ray for all squares.
			pub const fn generate_ray_se_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self::EMPTY; Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_ray_se_mask(i);
					i += 1;
				}
				arr
			}

			/// Computes a ray from `index` in south-west direction until board edge.
			#[inline]
			pub const fn compute_ray_sw_mask(index: usize) -> Self {
				Self::compute_ray_mask(index, -1, -1)
			}
			/// Generates a table of south-west ray for all squares.
			pub const fn generate_ray_sw_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self::EMPTY; Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_ray_sw_mask(i);
					i += 1;
				}
				arr
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
			
			/// Computes ascending diagonal mask (bottom-left → top-right) for square at `index`.
			#[inline(always)]
			pub const fn compute_diag_inc_mask(index: usize) -> Self {
				let (x0, y0) = Self::coords_from_index(index);
				let mut bb = Self::EMPTY;

				let mut x = x0+1;
				let mut y = y0+1;
				loop {
					if x >= Self::WIDTH || y >= Self::HEIGHT {
						break;
					}
					bb = bb.or_const(&Self::from_index(Self::index_from_coords(x, y)));

					x += 1;
					y += 1;
				}
				let mut x = x0;
				let mut y = y0;
				loop {
					bb = bb.or_const(&Self::from_index(Self::index_from_coords(x, y)));

					if x == 0 || y == 0 {
						break;
					}
					x -= 1;
					y -= 1;
				}
				bb
			}

			pub const fn generate_diag_inc_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self::EMPTY; Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_diag_inc_mask(i);
					i += 1;
				}
				arr
			}

			/// Computes descending diagonal mask (top-left → bottom-right) for square at `index`.
			#[inline(always)]
			pub const fn compute_diag_dec_mask(index: usize) -> Self {
				let (x0, y0) = Self::coords_from_index(index);
				let mut bb = Self::EMPTY;

				let mut x = x0 as i16 + 1;
				let mut y = y0 as i16 - 1;
				loop {
					if x >= Self::WIDTH as i16 || y < 0 {
						break;
					}
					bb = bb.or_const(&Self::from_index(Self::index_from_coords(x as u8, y as u8)));

					x += 1;
					y -= 1;
				}
				let mut x = x0 as i16;
				let mut y = y0 as i16;
				loop {
					if x < 0 || y >= Self::HEIGHT as i16 {
						break;
					}
					bb = bb.or_const(&Self::from_index(Self::index_from_coords(x as u8, y as u8)));

					x -= 1;
					y += 1;
				}
				bb
			}
			pub const fn generate_diag_dec_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self::EMPTY; Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_diag_dec_mask(i);
					i += 1;
				}
				arr
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
			/// Computes the bitboard mask the n north columns.
			pub const fn compute_north_cols_mask(n: u8) -> Self {
				let mut bb = Self::empty();
				let mut i=0;
				while i < n as usize {
					bb.or_assign_const(&Self::NORTH_BORDER.shr_const(Self::V_OFFSET * i));
					i += 1;
				}

				bb.not_assign_const();
				bb
			}
			/// Computes the bitboard mask the n south columns.
			const fn compute_south_cols_mask(n: u8) -> Self {
				let mut bb = Self::empty();
				let mut i=0;
				while i < n as usize {
					bb.or_assign_const(&Self::SOUTH_BORDER.shl_const(Self::V_OFFSET * i));
					i += 1;
				}

				bb.not_assign_const();
				bb
			}
			/// Computes the bitboard mask the n west columns.
			const fn compute_west_cols_mask(n: u8) -> Self {
				let mut bb = Self::empty();
				let mut i=0;
				while i < n as usize {
					bb.or_assign_const(&Self::WEST_BORDER.shl_const(Self::H_OFFSET * i));
					i += 1;
				}
				bb.not_assign_const();
				bb
			}
			/// Computes the bitboard mask the n east columns.
			const fn compute_east_cols_mask(n: u8) -> Self {
				let mut bb = Self::empty();
				let mut i=0;
				while i < n as usize {
					bb.or_assign_const(&Self::EAST_BORDER.shr_const(Self::H_OFFSET * i));
					i += 1;
				}

				bb.not_assign_const();
				bb
			}
			#[inline(always)]
			pub fn shifted(&self, dx: i16, dy: i16) -> Self {
				let mut res = self.clone_const();
				res.shift(dx, dy);
				res
			}
			#[inline(always)]
			pub fn shift(&mut self, dx: i16, dy: i16) {
				if Self::COL_MAJOR {
					if dy > 0 {
						self.and_assign_const(&Self::compute_north_cols_mask(dy as u8));
					} else if dy < 0 {
						self.and_assign_const(&Self::compute_south_cols_mask((-dy) as u8));
					}
				} else {
					if dx > 0 {
						self.and_assign_const(&Self::compute_east_cols_mask(dx as u8));
					} else if dx < 0 {
						self.and_assign_const(&Self::compute_west_cols_mask((-dx) as u8));
					}
				}
				let delta = dx as isize * Self::H_OFFSET as isize + dy as isize * Self::V_OFFSET as isize;
				if delta >= 0 {
					self.shl_assign_const(delta as usize);
				} else {
					self.shr_assign_const((-delta) as usize);
				}
			}
			
			/// Shift bitboard one square north (up).
			#[inline(always)]
			pub const fn shift_n(&mut self) {
				if Self::COL_MAJOR {
					self.and_assign_const(&Self::NO_WRAP_N_MASK);
				}
				self.shl_assign_const(Self::V_OFFSET);
			}
			/// Shift bitboard n square north (up).
			#[inline(always)]
			pub const fn shift_n_by(&mut self, n: u8) {
				if Self::COL_MAJOR {
					//TODO: precomputed mask
					self.and_assign_const(&Self::compute_north_cols_mask(n));
				}
				self.shl_assign_const(Self::V_OFFSET * n as usize);
			}
			/// Shifted bitboard one square north (up).
			#[inline(always)]
			pub const fn shifted_n(&self) -> Self {
				if Self::COL_MAJOR {
					self.and_const(&Self::NO_WRAP_N_MASK).shl_const(Self::V_OFFSET)
				} else {
					self.shl_const(Self::V_OFFSET)
				}
			}
			/// Shift bitboard one square south (down).
			#[inline(always)]
			pub const fn shift_s(&mut self) {
				if Self::COL_MAJOR {
					self.and_assign_const(&Self::NO_WRAP_S_MASK);
				}
				self.shr_assign_const(Self::V_OFFSET);
			}
			/// Shift bitboard n square south (down).
			#[inline(always)]
			pub const fn shift_s_by(&mut self, n: u8) {
				if Self::COL_MAJOR {
					//TODO: precomputed mask
					self.and_assign_const(&Self::compute_south_cols_mask(n));
				}
				self.shr_assign_const(Self::V_OFFSET * n as usize);
			}
			/// Shifted bitboard one square south (down).
			#[inline(always)]
			pub const fn shifted_s(&self) -> Self {
				if Self::COL_MAJOR {
					self.and_const(&Self::NO_WRAP_S_MASK).shr_const(Self::V_OFFSET)
				} else {
					self.shr_const(Self::V_OFFSET)
				}
			}
			/// Shift bitboard one square east (right).
			#[inline(always)]
			pub const fn shift_e(&mut self) {
				if !Self::COL_MAJOR {
					self.and_assign_const(&Self::NO_WRAP_E_MASK);
				}
				self.shl_assign_const(Self::H_OFFSET);
			}
			/// Shift bitboard n squares east (right).
			#[inline(always)]
			pub const fn shift_e_by(&mut self, n: u8) {
				if !Self::COL_MAJOR {
					//TODO: precomputed mask
					self.and_assign_const(&Self::compute_east_cols_mask(n));
				}
				self.shl_assign_const(Self::H_OFFSET * n as usize);
			}
			/// Shifted bitboard one square east (right).
			#[inline(always)]
			pub const fn shifted_e(&self) -> Self {
				if !Self::COL_MAJOR {
					self.and_const(&Self::NO_WRAP_E_MASK).shl_const(Self::H_OFFSET)
				} else {
					self.shl_const(Self::H_OFFSET)
				}
			}
			/// Shift bitboard one square west (left).
			#[inline(always)]
			pub const fn shift_w(&mut self) {
				if !Self::COL_MAJOR {
					self.and_assign_const(&Self::NO_WRAP_W_MASK);
				}
				self.shr_assign_const(Self::H_OFFSET);
			}
			/// Shift bitboard n squares west (left).
			#[inline(always)]
			pub const fn shift_w_by(&mut self, n: u8) {
				if !Self::COL_MAJOR {
					//TODO: precomputed mask
					self.and_assign_const(&Self::compute_west_cols_mask(n));
				}
				self.shr_assign_const(Self::H_OFFSET * n as usize);
			}
			/// Shifted bitboard one square west (left).
			#[inline(always)]
			pub const fn shifted_w(&self) -> Self {
				if !Self::COL_MAJOR {
					self.and_const(&Self::NO_WRAP_W_MASK).shr_const(Self::H_OFFSET)
				} else {
					self.shr_const(Self::H_OFFSET)
				}
			}
			const NE_OFFSET: isize = Self::V_OFFSET as isize + Self::H_OFFSET as isize;
			/// Shift bitboard one square north-east.
			#[inline(always)]
			pub const fn shift_ne(&mut self) {
				self.and_assign_const(&Self::NO_WRAP_NE_MASK);
				self.shl_assign_const(Self::NE_OFFSET as usize);
			}
			/// Shift bitboard n squares north-east.
			#[inline(always)]
			pub const fn shift_ne_by(&mut self, n: u8) {
				if Self::COL_MAJOR {
					self.and_assign_const(&Self::compute_north_cols_mask(n));
				} else {
					self.and_assign_const(&Self::compute_east_cols_mask(n));
				}
				self.shl_assign_const(Self::NE_OFFSET as usize * n as usize);
			}
			/// Shifted bitboard one square north-east.
			#[inline(always)]
			pub const fn shifted_ne(&self) -> Self {
				self.and_const(&Self::NO_WRAP_NE_MASK).shl_const(Self::NE_OFFSET as usize)
			}
			const NW_OFFSET: isize = Self::V_OFFSET as isize - Self::H_OFFSET as isize;
			/// Shift bitboard one square north-west.
			#[inline(always)]
			pub const fn shift_nw(&mut self) {
				self.and_assign_const(&Self::NO_WRAP_NW_MASK);
				if Self::NW_OFFSET >= 0 {
					self.shl_assign_const(Self::NW_OFFSET as usize);
				} else {
					self.shr_assign_const((-Self::NW_OFFSET) as usize);
				}
			}
			/// Shift bitboard n squares north-west.
			#[inline(always)]
			pub const fn shift_nw_by(&mut self, n: u8) {
				if Self::COL_MAJOR {
					self.and_assign_const(&Self::compute_north_cols_mask(n));
				} else {
					self.and_assign_const(&Self::compute_west_cols_mask(n));
				}
				if Self::NW_OFFSET >= 0 {
					self.shl_assign_const(Self::NW_OFFSET as usize * n as usize);
				} else {
					self.shr_assign_const((-Self::NW_OFFSET) as usize * n as usize);
				}
			}
			/// Shifted bitboard one square north-west.
			#[inline(always)]
			pub const fn shifted_nw(&self) -> Self {
				if Self::NW_OFFSET >= 0 {
					self.and_const(&Self::NO_WRAP_NW_MASK).shl_const(Self::NW_OFFSET as usize)
				} else {
					self.and_const(&Self::NO_WRAP_NW_MASK).shr_const((-Self::NW_OFFSET) as usize)
				}
			}
			const SE_OFFSET: isize = Self::H_OFFSET as isize - Self::V_OFFSET as isize;
			/// Shift bitboard one square south-east.
			#[inline(always)]
			pub const fn shift_se(&mut self) {
				if Self::SE_OFFSET >= 0 {
					self.and_assign_const(&Self::NO_WRAP_SE_MASK);
					self.shl_assign_const(Self::SE_OFFSET as usize);
				} else {
					self.and_assign_const(&Self::NO_WRAP_SE_MASK);
					self.shr_assign_const((-Self::SE_OFFSET) as usize);
				}
			}
			/// Shift bitboard n squares south-east.
			#[inline(always)]
			pub const fn shift_se_by(&mut self, n: u8) {
				if Self::COL_MAJOR {
					self.and_assign_const(&Self::compute_south_cols_mask(n));
				} else {
					self.and_assign_const(&Self::compute_east_cols_mask(n));
				}
				if Self::SE_OFFSET >= 0 {
					self.shl_assign_const(Self::SE_OFFSET as usize * n as usize);
				} else {
					self.shr_assign_const(-Self::SE_OFFSET as usize * n as usize);
				}
				
			}
			/// Shifted bitboard one square south-east.
			#[inline(always)]
			pub const fn shifted_se(&self) -> Self {
				if Self::SE_OFFSET >= 0 {
					self.and_const(&Self::NO_WRAP_SE_MASK).shl_const(Self::SE_OFFSET as usize)
				} else {
					self.and_const(&Self::NO_WRAP_SE_MASK).shr_const((-Self::SE_OFFSET) as usize)
				}
			}
			const SW_OFFSET: isize = -(Self::H_OFFSET as isize + Self::V_OFFSET as isize);
			/// Shift bitboard one square south-west.
			#[inline(always)]
			pub const fn shift_sw(&mut self) {
				self.and_assign_const(&Self::NO_WRAP_SW_MASK);
				self.shr_assign_const((-Self::SW_OFFSET) as usize);
			}
			/// Shift bitboard n squares south-west.
			#[inline(always)]
			pub const fn shift_sw_by(&mut self, n: u8) {
				if Self::COL_MAJOR {
					self.and_assign_const(&Self::compute_south_cols_mask(n));
				} else {
					self.and_assign_const(&Self::compute_west_cols_mask(n));
				}
				self.shr_assign_const(-Self::SW_OFFSET as usize * n as usize);
				
			}
			/// Shifted bitboard one square south-west.
			#[inline(always)]
			pub const fn shifted_sw(&self) -> Self {
				self.and_const(&Self::NO_WRAP_SW_MASK).shr_const((-Self::SW_OFFSET) as usize)
			}
			/// Return the dilated board
			pub const fn dilated(&self) -> Self {
				let mut	res = self.clone_const();
				res.or_assign_const(&self.shifted_e());
				res.or_assign_const(&self.shifted_ne());
				res.or_assign_const(&self.shifted_n());
				res.or_assign_const(&self.shifted_nw());
				res.or_assign_const(&self.shifted_w());
				res.or_assign_const(&self.shifted_sw());
				res.or_assign_const(&self.shifted_s());
				res.or_assign_const(&self.shifted_se());
				res
			}
			/// Return the eroded board
			pub const fn eroded(&self) -> Self {
				let mut	res = self.clone_const();
				res.and_assign_const(&self.shifted_e());
				res.and_assign_const(&self.shifted_ne());
				res.and_assign_const(&self.shifted_n());
				res.and_assign_const(&self.shifted_nw());
				res.and_assign_const(&self.shifted_w());
				res.and_assign_const(&self.shifted_sw());
				res.and_assign_const(&self.shifted_s());
				res.and_assign_const(&self.shifted_se());
				res
			}
			/// Return all neighbors of any stone in the bitboard
			pub const fn neighbors_of_any(&self) -> Self {
				let mut nei = self.dilated();
				nei.and_const(&self.not_const());
				nei
			}
			/// A Mask to prevent wrapping during shifts.
			pub const NO_WRAP_N_MASK : Self = Self::row_mask(Self::HEIGHT - 1).not_const();
			/// A Mask to prevent wrapping during shifts.
			pub const NO_WRAP_S_MASK : Self = Self::row_mask(0).not_const();
			/// A Mask to prevent wrapping during shifts.
			pub const NO_WRAP_E_MASK : Self = Self::col_mask(Self::WIDTH - 1).not_const();
			/// A Mask to prevent wrapping during shifts.
			pub const NO_WRAP_W_MASK : Self = Self::col_mask(0).not_const();
			/// A Mask to prevent wrapping during shifts.
			pub const NO_WRAP_NE_MASK : Self = Self::NO_WRAP_N_MASK.and_const(&Self::NO_WRAP_E_MASK);
			/// A Mask to prevent wrapping during shifts.
			pub const NO_WRAP_NW_MASK : Self = Self::NO_WRAP_N_MASK.and_const(&Self::NO_WRAP_W_MASK);
			/// A Mask to prevent wrapping during shifts.
			pub const NO_WRAP_SE_MASK : Self = Self::NO_WRAP_S_MASK.and_const(&Self::NO_WRAP_E_MASK);
			/// A Mask to prevent wrapping during shifts.
			pub const NO_WRAP_SW_MASK : Self = Self::NO_WRAP_S_MASK.and_const(&Self::NO_WRAP_W_MASK);

			pub const fn detect_pattern_h(&self, mut mask: u64) -> Self {
				let mut res = Self::FULL;

				let mut shifted = self.clone_const();
				let mut current_nb_shift = 0;
				while mask != 0 {
					let lsb = mask.trailing_zeros();
					mask &= mask - 1;

					//let mut i = current_nb_shift;
					//while i < lsb {
					//	shifted.shift_e();
					//	i += 1;
					//	current_nb_shift+=1;
					//}
					shifted.shift_e_by((lsb- current_nb_shift) as u8);
					current_nb_shift = lsb;
					res.and_assign_const(&shifted);
				}

				res
			}
			pub const fn detect_pattern_v(&self, mut mask: u64) -> Self {
				let mut res = Self::FULL;

				let mut shifted = self.clone_const();
				let mut current_nb_shift = 0;
				while mask != 0 {
					let lsb = mask.trailing_zeros();
					mask &= mask - 1;

					//let mut i = current_nb_shift;
					//while i < lsb {
					//	shifted.shift_n();
					//	i += 1;
					//	current_nb_shift+=1;
					//}
					shifted.shift_n_by((lsb- current_nb_shift) as u8);
					current_nb_shift = lsb;

					res.and_assign_const(&shifted);
				}

				res
			}
			pub const fn detect_pattern_diag_inc(&self, mut mask: u64) -> Self {
				let mut res = Self::FULL;

				let mut shifted = self.clone_const();
				let mut current_nb_shift = 0;
				while mask != 0 {
					let lsb = mask.trailing_zeros();
					mask &= mask - 1;

					//let mut i = current_nb_shift;
					//while i < lsb {
					//	shifted.shift_ne();
					//	i += 1;
					//	current_nb_shift+=1;
					//}
					shifted.shift_ne_by((lsb- current_nb_shift) as u8);
					current_nb_shift = lsb;

					res.and_assign_const(&shifted);
				}

				res
			}
			pub const fn detect_pattern_diag_dec(&self, mut mask: u64) -> Self {
				let mut res = Self::FULL;

				let mut shifted = self.clone_const();
				let mut current_nb_shift = 0;
				while mask != 0 {
					let lsb = mask.trailing_zeros();
					mask &= mask - 1;

					//let mut i = current_nb_shift;
					//while i < lsb {
					//	shifted.shift_se();
					//	i += 1;
					//	current_nb_shift+=1;
					//}
					shifted.shift_se_by((lsb- current_nb_shift) as u8);
					current_nb_shift = lsb;

					res.and_assign_const(&shifted);
				}

				res
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
						//temp = temp.and_const(&shifted);
						temp.and_assign_const(&shifted);
					} else {
						//temp &= temp >> Self::H_OFFSET;
						let shifted = temp.shr_const(Self::H_OFFSET as usize);
						//temp = temp.and_const(&shifted);
						temp.and_assign_const(&shifted);
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
						//temp = temp.and_const(&shifted);
						temp.and_assign_const(&shifted);
					} else {
						//temp &= temp >> Self::V_OFFSET;
						let shifted = temp.shr_const(Self::V_OFFSET as usize);
						//temp = temp.and_const(&shifted);
						temp.and_assign_const(&shifted);
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
					//temp = temp.and_const(&shifted);
					temp.and_assign_const(&shifted);

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
					//temp = temp.and_const(&shifted);
					temp.and_assign_const(&shifted);
					i += 1;
				}
				temp.any()
			}

			#[inline]
			const fn get_aligned_dir_const<const N: usize>(
				&self,
				offset: usize,
				mask: Option<Self>,
			) -> Self {
				//if N <= 1 {
				//	return N == 0 || self.any();
				//}

				let mut temp = self.clone_const();
				let mut built = 1;
				let mut shift = offset;

				while built * 2 <= N {
					let shifted = match &mask {
						Some(m) => temp.and_const(m).shr_const(shift),
						None => temp.shr_const(shift),
					};

					temp = temp.and_const(&shifted);
					built *= 2;
					shift <<= 1;
				}

				let mut remaining = N - built;
				while remaining > 0 {
					let shifted = match &mask {
						Some(m) => temp.and_const(m).shr_const(offset),
						None => temp.shr_const(offset),
					};

					temp = temp.and_const(&shifted);
					remaining -= 1;
				}

				temp
			}
			#[inline]
			pub fn has_aligned2<const N: usize>(&self) -> bool {
				self.has_aligned_horizontal2::<N>()
					|| self.has_aligned_vertical2::<N>()
					|| self.has_aligned_diag_dec2::<N>()
					|| self.has_aligned_diag_inc2::<N>()
			}

			#[inline]
			pub const fn has_aligned_horizontal2<const N: usize>(&self) -> bool {
				if !Self::COL_MAJOR {
					self.get_aligned_dir_const::<N>(
						Self::H_OFFSET as usize,
						Some(Self::WEST_BORDER.not_const()),
					).any()
				} else {
					self.get_aligned_dir_const::<N>(
						Self::H_OFFSET as usize,
						None,
					).any()
				}
			}
			#[inline]
			pub const fn has_aligned_vertical2<const N: usize>(&self) -> bool {
				if Self::COL_MAJOR {
					self.get_aligned_dir_const::<N>(
						Self::V_OFFSET as usize,
						Some(Self::SOUTH_BORDER.not_const()),
					).any()
				} else {
					self.get_aligned_dir_const::<N>(
						Self::V_OFFSET as usize,
						None,
					).any()
				}
			}
			#[inline]
			pub const fn has_aligned_diag_dec2<const N: usize>(&self) -> bool {
				self.get_aligned_dir_const::<N>(
					Self::DIAG_DEC_OFFSET as usize,
					Some(Self::EAST_BORDER.not_const()),
				).any()
			}
			#[inline]
			pub const fn has_aligned_diag_inc2<const N: usize>(&self) -> bool {
				self.get_aligned_dir_const::<N>(
					Self::DIAG_INC_OFFSET as usize,
					Some(Self::WEST_BORDER.not_const()),
				).any()
			}

			#[inline]
			pub const fn has_aligned<const N: usize>(&self) -> bool {
				if N == 0 { return true; }
				if N == 1 { return self.any(); }

				self.has_aligned_horizontal::<N>() ||
					self.has_aligned_vertical::<N>() ||
					self.has_aligned_diag_dec::<N>() ||
					self.has_aligned_diag_inc::<N>()
			}
			#[inline]
			pub const fn has_aligned_horizontal<const N: usize>(&self) -> bool {
				if N == 0 { return true; }
				if N == 1 { return self.any(); }

				let mut temp = Self(self.0);//self.clone_const();
				let mut i=1;
				while i < N {
					if !Self::COL_MAJOR {
						//let west_mask = !Self::WEST_BORDER.storage();
						//temp&=(temp&west_mask)>>Self::H_OFFSET;
						let mask = Self::WEST_BORDER.not_const();
						let shifted = temp.and_const(&mask).shr_const(Self::H_OFFSET as usize);
						//temp = temp.and_const(&shifted);
						temp.and_assign_const(&shifted);
					} else {
						//temp &= temp >> Self::H_OFFSET;
						let shifted = temp.shr_const(Self::H_OFFSET as usize);
						//temp = temp.and_const(&shifted);
						temp.and_assign_const(&shifted);
					}
					i += 1;
				}
				temp.any()
			}
			#[inline]
			pub const fn has_aligned_vertical<const N: usize>(&self) -> bool {
				if N == 0 { return true; }
				if N == 1 { return self.any(); }

				let mut temp = Self(self.0);//self.clone_const();
				let mut i=1;
				while i < N {
					if Self::COL_MAJOR {
						//let south_mask = !Self::SOUTH_BORDER.storage();
						//temp&=(temp&south_mask)>>Self::V_OFFSET;
						let mask = Self::SOUTH_BORDER.not_const();
						let shifted = temp.and_const(&mask).shr_const(Self::V_OFFSET as usize);
						//temp = temp.and_const(&shifted);
						temp.and_assign_const(&shifted);
					} else {
						//temp &= temp >> Self::V_OFFSET;
						let shifted = temp.shr_const(Self::V_OFFSET as usize);
						//temp = temp.and_const(&shifted);
						temp.and_assign_const(&shifted);
					}
					i += 1;
				}
				temp.any()
			}
			#[inline]
			pub const fn has_aligned_diag_dec<const N: usize>(&self) -> bool {
				if N <= 1 { return N == 1 && self.any() || N == 0; }
				let mut temp = Self(self.0);//self.clone_const();
				let mut i = 1;
				let mask = Self::EAST_BORDER.not_const(); 

				while i < N {
					// temp &= (temp & mask) >> offset
					let can_slide = temp.and_const(&mask);
					let shifted = can_slide.shr_const(Self::DIAG_DEC_OFFSET as usize);
					//temp = temp.and_const(&shifted);
					temp.and_assign_const(&shifted);

					i += 1;
				}
				temp.any()
			}

			#[inline]
			pub const fn has_aligned_diag_inc<const N: usize>(&self) -> bool {
				if N <= 1 { return N == 1 && self.any() || N == 0; }
				let mut temp = Self(self.0);//self.clone_const();
				let mut i = 1;
				let mask = Self::WEST_BORDER.not_const();
				while i < N {
					// temp = temp & ((temp & mask) >> offset)
					let shifted = temp.and_const(&mask).shr_const(Self::DIAG_INC_OFFSET as usize);
					//temp = temp.and_const(&shifted);
					temp.and_assign_const(&shifted);
					i += 1;
				}
				temp.any()
			}

			#[inline]
			pub fn count_aligned<const N: usize>(&self) -> u32 {
				self.count_aligned_horizontal::<N>()
					+ self.count_aligned_vertical::<N>()
					+ self.count_aligned_diag_dec::<N>()
					+ self.count_aligned_diag_inc::<N>()
			}
			#[inline]
			pub fn count_aligned_horizontal<const N: usize>(&self) -> u32 {
				if !Self::COL_MAJOR {
					self.get_aligned_dir_const::<N>(
						Self::H_OFFSET as usize,
						Some(Self::WEST_BORDER.not_const()),
					).count()
				} else {
					self.get_aligned_dir_const::<N>(
						Self::H_OFFSET as usize,
						None,
					).count()
				}
			}
			#[inline]
			pub fn count_aligned_vertical<const N: usize>(&self) -> u32 {
				if Self::COL_MAJOR {
					self.get_aligned_dir_const::<N>(
						Self::V_OFFSET as usize,
						Some(Self::SOUTH_BORDER.not_const()),
					).count()
				} else {
					self.get_aligned_dir_const::<N>(
						Self::V_OFFSET as usize,
						None,
					).count()
				}
			}
			#[inline]
			pub fn count_aligned_diag_dec<const N: usize>(&self) -> u32 {
				self.get_aligned_dir_const::<N>(
					Self::DIAG_DEC_OFFSET as usize,
					Some(Self::EAST_BORDER.not_const()),
				).count()
			}
			#[inline]
			pub fn count_aligned_diag_inc<const N: usize>(&self) -> u32 {
				self.get_aligned_dir_const::<N>(
					Self::DIAG_INC_OFFSET as usize,
					Some(Self::WEST_BORDER.not_const()),
				).count()
			}
		}
	}
}
