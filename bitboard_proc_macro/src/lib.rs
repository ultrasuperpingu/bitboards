
use proc_macro::TokenStream;
use quote::quote;
use syn::{
	parse_macro_input, ItemStruct, Fields,
};

#[proc_macro_attribute]
pub fn bitboard(attr: TokenStream, item: TokenStream) -> TokenStream {
	use syn::{parse::Parser, Meta, Expr};
	
	let parser = syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated;
	let metas = parser.parse(attr).unwrap();
	
	let mut width = None;
	let mut height = None;
	let mut col_major = None;
	
	for meta in metas {
		if let Meta::NameValue(nv) = meta {
			let ident = nv.path.get_ident().unwrap().to_string();
			
			match ident.as_str() {
				"width" => {
					if let Expr::Lit(expr_lit) = nv.value {
						if let syn::Lit::Int(lit_int) = expr_lit.lit {
							width = Some(lit_int.base10_parse::<usize>().unwrap());
						}
					}
				}
				"height" => {
					if let Expr::Lit(expr_lit) = nv.value {
						if let syn::Lit::Int(lit_int) = expr_lit.lit {
							height = Some(lit_int.base10_parse::<usize>().unwrap());
						}
					}
				}
				"col_major" => {
					if let Expr::Lit(expr_lit) = nv.value {
						if let syn::Lit::Bool(lit_bool) = expr_lit.lit {
							col_major = Some(lit_bool.value);
						}
					}
				}
				_ => {}
			}
		}
	}
	
	
	let width = width.expect("missing width");
	let height = height.expect("missing height");
	let col_major = col_major.unwrap_or(false);
	
	let input_struct = parse_macro_input!(item as ItemStruct);
	let struct_attrs = input_struct.attrs.clone();
	let struct_vis = input_struct.vis.clone();
	let struct_ident = input_struct.ident.clone();
	
	let total_bits = width * height;
	let array_bytes = total_bits.div_ceil(64);
	let is_storage_array = total_bits > 128; 
	let storage_ty = if total_bits <= 8 {
		quote! { u8 }
	} else if total_bits <= 16 {
		quote! { u16 }
	} else if total_bits <= 32 {
		quote! { u32 }
	} else if total_bits <= 64 {
		quote! { u64 }
	} else if total_bits <= 128 {
		quote! { u128 }
	} else {
		quote! { [u64; #array_bytes] }
	};
	
	let expanded_struct = match &input_struct.fields {
		Fields::Unit => {
			quote! {
				struct #struct_ident(pub(crate) #storage_ty);
			}
		}
		Fields::Named(_) => {
			panic!("Named structs not supported yet. Only Unit struct");
		}
		Fields::Unnamed(_) => {
			panic!("Tuple structs not supported yet. Only Unit struct");
		}
	};
	let width_u8 = width as u8;
	let height_u8 = height as u8;
	let bitboard_impl_common = quote!{
		pub const WIDTH: u8 = #width_u8;
		pub const HEIGHT: u8 = #height_u8;
		pub const NB_SQUARES: usize = Self::WIDTH as usize * Self::HEIGHT as usize;
		pub const COL_MAJOR: bool = #col_major;
		pub const H_OFFSET: usize = if Self::COL_MAJOR { Self::HEIGHT as usize } else { 1 };
		pub const V_OFFSET: usize = if Self::COL_MAJOR { 1 } else { Self::WIDTH as usize };
		pub const DIAG_INC_OFFSET: u8 = Self::WIDTH - 1;
		pub const DIAG_DEC_OFFSET: u8 = Self::WIDTH + 1;
		
	};
	let bits = width * height;
	
	let full_mask = if bits >= 128 {
		u128::MAX
	} else {
		(1u128 << bits) - 1
	};
	let mut left_mask: u128 = 0;
	let mut right_mask: u128 = 0;
	let mut bottom_mask: u128 = 0;
	let mut top_mask: u128 = 0;
	let mut center_mask: u128 = 0;
	let mut odd_mask: u128 = 0;
	let mut even_mask: u128 = 0;
	let mut corners_mask: u128 = 0;
	let mut north_mask: u128 = 0;
	let mut south_mask: u128 = 0;
	let mut west_mask: u128 = 0;
	let mut east_mask: u128 = 0;

	if !is_storage_array {
		let idx = |x: usize, y: usize| -> usize {
			if col_major {
				x * height + y
			} else {
				y * width + x
			}
		};
		
		let cx0 = (width  - 1) / 2;
		let cy0 = (height - 1) / 2;
		let cx1 = width  / 2;
		let cy1 = height / 2;
		
		let is_center = |x: usize, y: usize| -> bool {
			match (width % 2, height % 2) {
				(1, 1) => x == cx0 && y == cy0,
				(1, 0) => x == cx0 && (y == cy0 || y == cy1),
				(0, 1) => y == cy0 && (x == cx0 || x == cx1),
				(0, 0) => (x == cx0 || x == cx1) && (y == cy0 || y == cy1),
				_ => false,
			}
		};
		let width_odd_offset = if width%2 == 0 {0} else {1};
		let height_odd_offset = if height%2 == 0 {0} else {1};
		for y in 0..height {
			for x in 0..width {
				let bit = 1u128 << idx(x, y);
				
				if x == 0            { left_mask   |= bit; }
				if x == width - 1    { right_mask  |= bit; }
				if y == 0            { bottom_mask |= bit; }
				if y == height - 1   { top_mask    |= bit; }
				if is_center(x, y)   { center_mask |= bit; }
				
				if (x + y) % 2 == 0  { even_mask   |= bit; }
				else                 { odd_mask    |= bit; }
				
				if (x == 0 || x == width - 1) &&
				(y == 0 || y == height - 1)
				{
					corners_mask |= bit;
				}
				
				if y < height / 2    { south_mask  |= bit; }
				else if y - height_odd_offset >= height / 2 { north_mask |= bit; }
				
				if x < width / 2     { west_mask   |= bit; }
				else if x - width_odd_offset >= width / 2 { east_mask  |= bit; }
			}
		}
	}
	
	let pext_pdep = if total_bits > 64 {
		quote! {
			#[inline]
			fn lsb(&self) -> u32 {
				self.0.trailing_zeros()
			}
			#[inline]
			fn pop_lsb(&mut self) -> u32 {
				#[cfg(target_feature = "bmi2")]
				unsafe {
					let low = self.0 as u64;
					
					if low != 0 {
						let idx = std::arch::x86_64::_tzcnt_u64(low);
						let new_low = std::arch::x86_64::_blsr_u64(low);
						
						let high = self.0 & (!0u128 << 64);
						self.0 = high | new_low as u128;
						
						return idx as u32;
					}
					
					let high = (self.0 >> 64) as u64;
					let idx = std::arch::x86_64::_tzcnt_u64(high);
					let new_high = std::arch::x86_64::_blsr_u64(high);
					
					self.0 = (new_high as u128) << 64;
					
					return (idx + 64) as u32;
				}
				#[cfg(not(target_feature = "bmi2"))]
				{
					// fallback
					let low = self.0 as u64;
					
					if low != 0 {
						let idx = low.trailing_zeros();
						self.0 &= self.0 - 1;
						return idx;
					}
					
					let high = (self.0 >> 64) as u64;
					let idx = high.trailing_zeros();
					self.0 &= self.0 - 1;
					idx + 64
				}
			}
			
			#[inline]
			fn pext(&self, mask: &Self) -> Self::Storage {
				#[cfg(target_feature = "bmi2")]
				unsafe {
					let lo  = self.0 as u64;
					let hi  = (self.0 >> 64) as u64;
					
					let mlo = mask.0 as u64;
					let mhi = (mask.0 >> 64) as u64;
					
					let lo_res = std::arch::x86_64::_pext_u64(lo, mlo);
					let hi_res = std::arch::x86_64::_pext_u64(hi, mhi);
					
					let shift = mlo.count_ones() as u32;
					
					return ((hi_res as u128) << shift) | (lo_res as u128);
				}
				#[cfg(not(target_feature = "bmi2"))]
				{
					let mut res: u128 = 0;
					let mut bit: u128 = 1;
					let mut m = mask.0;
					let mut v = self.0;
					
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
					
					let lo_mask = self.0 as u64;
					let hi_mask = (self.0 >> 64) as u64;
					
					let lo_count = lo_mask.count_ones() as u32;
					
					let src_lo = src as u64;
					let src_hi = (src >> lo_count) as u64;
					
					let lo_res = std::arch::x86_64::_pdep_u64(src_lo, lo_mask);
					let hi_res = std::arch::x86_64::_pdep_u64(src_hi, hi_mask);
					
					return Self((lo_res as u128) | ((hi_res as u128) << 64));
				}
				#[cfg(not(target_feature = "bmi2"))]
				{
					
					// fallback
					let mut res: u128 = 0;
					let mut bit: u128 = 1;
					let mut m = compressed;
					let mut v = self.0;
					
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
					Self(res)
				}
			}
			
		}
	} else {
		quote! {
			#[inline(always)]
			fn lsb(&self) -> u32 {
				#[cfg(target_feature = "bmi1")]
				unsafe {
					std::arch::x86_64::_tzcnt_u64(self.0 as u64) as u32
				}
				#[cfg(not(target_feature = "bmi1"))]
				self.0.trailing_zeros()
			}
			#[inline]
			#[allow(unreachable_code)]
			fn pop_lsb(&mut self) -> u32 {
				#[cfg(target_feature = "bmi1")]
				unsafe {
					let idx = std::arch::x86_64::_tzcnt_u64(self.0 as u64);
					self.0 = std::arch::x86_64::_blsr_u64(self.0 as u64) as #storage_ty;
					return idx as u32;
				}
				#[cfg(not(target_feature = "bmi1"))]
				{
					// fallback
					let idx = self.0.trailing_zeros();
					self.0 &= self.0.wrapping_sub(1);
					idx
				}
			}
			#[inline]
			fn pext(&self, mask: &Self) -> Self::Storage {
				#[cfg(target_feature = "bmi2")]
				unsafe {
					return std::arch::x86_64::_pext_u64(self.0 as u64, mask.0 as u64) as #storage_ty;
				}
				#[cfg(not(target_feature = "bmi2"))]
				{
					#[cfg(debug_assertions)]
					eprintln!("pext fallback");
					let mut res: #storage_ty = 0;
					let mut bit: #storage_ty = 1;
					let mut m = mask.0;
					let mut v = self.0;
					
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
					return Self(std::arch::x86_64::_pdep_u64(compressed as u64, self.0 as u64) as #storage_ty);
				}
				#[cfg(not(target_feature = "bmi2"))]
				{
					#[cfg(debug_assertions)]
					eprintln!("pdep_bmi2 fallback");
					
					let mut res: #storage_ty = 0;
					let mut bit: #storage_ty = 1;
					let mut m = self.0;
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
					Self(res)
				}
			}
		}
	};
	
	let derive_bitboard_mask_res  = bitboard_mask_int_impl(&struct_ident);
	let impl_int = quote! {
		impl bitboard::Bitboard for #struct_ident {
			type Storage = #storage_ty;
			fn width(&self) -> u8 {
				Self::WIDTH as u8
			}
			fn height(&self) -> u8 {
				Self::HEIGHT as u8
			}
			fn col_major(&self) -> bool {
				Self::COL_MAJOR
			}
			#[inline(always)]
			fn is_empty(&self) -> bool {
				self.0 == 0
			}
			#[inline(always)]
			fn count(&self) -> u32 {
				self.0.count_ones()
			}
			#[inline(always)]
			fn intersects(&self, other: &Self) -> bool {
				self.0 & other.0 != 0
			}
			#[inline(always)]
			fn any(&self) -> bool {
				self.0 != 0
			}
			#[inline(always)]
			fn storage(&self) -> &Self::Storage {
				&self.0
			}
			#[inline(always)]
			fn storage_mut(&mut self) -> &mut Self::Storage {
				&mut self.0
			}
			#[inline(always)]
			fn get_at_index(&self, idx: usize) -> bool {
				(self.0 & (1 << idx)) != 0
			}
			
			#[inline(always)]
			fn set_at_index(&mut self, idx: usize) {
				self.0 |= 1 << idx;
			}
			#[inline(always)]
			fn reset_at_index(&mut self, idx: usize) {
				self.0 &= !(1 << idx);
			}
			#[inline(always)]
			fn set_value_at_index(&mut self, idx: usize, val: bool) {
				if val {
					self.0 |= 1 << idx;
				} else {
					self.0 &= !(1 << idx);
				}
			}
			#[inline(always)]
			fn toggle_at_index(&mut self, idx: usize) {
				self.0 ^= 1 << idx;
			}
			#[inline(always)]
			fn get(&self, x: u8, y: u8) -> bool {
				let idx = Self::index_from_coords(x, y);
				self.get_at_index(idx)
			}
			/// Sets the bit at coordinates `(x, y)`.
			#[inline(always)]
			fn set_value(&mut self, x: u8, y: u8, val: bool) {
				let idx = Self::index_from_coords(x, y);
				self.set_value_at_index(idx, val)
			}
			#[inline(always)]
			fn set(&mut self, x: u8, y: u8) {
				let idx = Self::index_from_coords(x, y);
				self.set_at_index(idx)
			}
			#[inline(always)]
			fn reset(&mut self, x: u8, y: u8) {
				let idx = Self::index_from_coords(x, y);
				self.reset_at_index(idx)
			}
			#[inline(always)]
			fn flipped(&self) -> Self {
				Self::from_storage(!self.0)
			}

			#pext_pdep
			
			#[inline(always)]
			fn extract_row(&self, y: u8) -> Self::Storage {
				let mask = Self::row_mask(y);
				self.pext(&mask)
			}
			#[inline(always)]
			fn extract_col(&self, x: u8) -> Self::Storage {
				let mask = Self::col_mask(x);
				self.pext(&mask)
			}
			#[inline(always)]
			fn insert_row(&mut self, y: u8, row_bits: Self::Storage) {
				let mask = Self::row_mask(y);

				let new_row = mask.pdep(row_bits);
				let cleared = *self & !mask;

				*self = cleared | new_row;
			}

			#[inline(always)]
			fn insert_col(&mut self,x: u8, col_bits: Self::Storage) {
				let mask = Self::col_mask(x);

				let new_col = mask.pdep(col_bits);
				let cleared = *self & !mask;

				*self = cleared | new_col;
			}
			
		}

		impl #struct_ident {
			#bitboard_impl_common
			pub const EMPTY: Self = Self(0);
			pub const FULL: Self = Self(#full_mask as #storage_ty);
			#[inline(always)]
			fn new() -> Self {
				Self::empty()
			}

			#[inline(always)]
			pub const fn empty() -> Self {
				Self(0)
			}

			#[inline(always)]
			pub const fn from_storage(v: <Self as bitboard::Bitboard>::Storage) -> Self {
				Self(v)
			}
			#[inline(always)]
			pub const fn from_index(idx: usize) -> Self {
				Self(1 << idx)
			}
			#[inline(always)]
			pub const fn from_coords(x: u8, y: u8) -> Self {
				Self::from_index(Self::index_from_coords(x,y))
			}
			#[inline(always)]
			pub const fn storage(&self) -> <Self as bitboard::Bitboard>::Storage {
				self.0
			}
			#[inline(always)]
			pub const fn flipped(&self) -> Self {
				Self::from_storage(!self.0 & Self::FULL.storage())
			}
			#[inline(always)]
			pub const fn get_at_index(&self, idx: usize) -> bool {
				(self.0 & (1 << idx)) != 0
			}
			#[inline(always)]
			pub const fn set_at_index(&mut self, idx: usize) {
				self.0 |= 1 << idx;
			}
			#[inline]
			pub const fn coords_from_index(i: usize) -> (u8, u8) {
				if Self::COL_MAJOR {
					((i / Self::HEIGHT as usize) as u8, (i % Self::HEIGHT as usize) as u8)
				} else {
					((i % Self::WIDTH as usize) as u8, (i / Self::WIDTH as usize) as u8)
				}
			}
			#[inline]
			pub const fn index_from_coords(x: u8, y: u8) -> usize {
				if Self::COL_MAJOR {
					x as usize * Self::HEIGHT as usize + y as usize
				} else {
					y as usize * Self::WIDTH as usize + x as usize
				}
			}
			pub fn generate_sliding_attacks_table(offsets: &[(i8, i8)]) -> [#struct_ident; Self::NB_SQUARES] 
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
			pub const fn generate_jump_attacks_table(offsets: &[(i8, i8)]) -> [#struct_ident; Self::NB_SQUARES] {
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
			pub const RAY_BETWEEN_MASKS: [[Self; Self::NB_SQUARES]; Self::NB_SQUARES] = Self::generate_ray_between_table();
			#[inline(always)]
			pub fn ray_between_mask(from: usize, to: usize) -> Self {
				Self::RAY_BETWEEN_MASKS[from][to]
			}
			pub const fn generate_ray_between_table() -> [[Self; Self::NB_SQUARES]; Self::NB_SQUARES] {
				let mut table = [[Self::from_storage(0); Self::NB_SQUARES]; Self::NB_SQUARES];
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

			pub const fn compute_ray_between_mask(from: usize, to: usize) -> Self {
				let (fx, fy) = Self::coords_from_index(from);
				let (tx, ty) = Self::coords_from_index(to);

				// Vérification d’alignement
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
					bb = Self::from_storage(
						bb.storage() |
						Self::from_coords(x as u8, y as u8).storage()
					);
					x += dx;
					y += dy;
				}

				bb
			}
		}
		impl #struct_ident {
			pub fn all_subsets(&self) -> Vec<Self>
			{
				let mut subsets = Vec::new();
				let zero = Self::empty();

				let mut subset = zero.clone();

				loop {
					subsets.push(subset.clone());
					subset = Self::from_storage(subset.0.wrapping_sub(self.0)) & *self;
					if subset == zero {
						break;
					}
				}

				subsets
			}
		}
		impl #struct_ident {
			pub const BORDER: Self = Self((#left_mask | #right_mask | #top_mask | #bottom_mask) as #storage_ty);
			pub const WEST_BORDER: Self = Self(#left_mask as #storage_ty);
			pub const EAST_BORDER: Self = Self(#right_mask as #storage_ty);
			pub const NORTH_BORDER: Self = Self(#top_mask as #storage_ty);
			pub const SOUTH_BORDER: Self = Self(#bottom_mask as #storage_ty);
			pub const CENTER: Self = Self(#center_mask as #storage_ty);
			pub const ODD_SQUARES: Self   = Self(#odd_mask   as #storage_ty);
			pub const EVEN_SQUARES: Self  = Self(#even_mask  as #storage_ty);
			pub const CORNERS: Self       = Self(#corners_mask as #storage_ty);
			pub const NORTH: Self         = Self(#north_mask as #storage_ty);
			pub const SOUTH: Self         = Self(#south_mask as #storage_ty);
			pub const WEST: Self          = Self(#west_mask  as #storage_ty);
			pub const EAST: Self          = Self(#east_mask  as #storage_ty);
			
			/*fn flip(&mut self) {
			self.0 = !self.0;
			self.0 &= (1 << #total_bits) - 1;
			}
			fn mask_flip(&mut self, mask: Self) {
			self.0 ^= mask.0;
			}*/
			#[inline(always)]
			fn extract_diag_inc(&self, index: usize) -> <Self as bitboard::Bitboard>::Storage {
				self.pext(&Self::compute_diag_inc_mask(index))
			}
			
			#[inline(always)]
			fn insert_diag_inc(&mut self, index: usize, bits: <Self as bitboard::Bitboard>::Storage) {
				let mask = Self::compute_diag_inc_mask(index);
				let new = mask.pdep(bits);
				let cleared = Self::from_storage(self.storage() & !mask.storage());
				*self = cleared | new;
			}
			#[inline(always)]
			fn extract_diag_dec(&self, index: usize) -> <Self as bitboard::Bitboard>::Storage {
				self.pext(&Self::compute_diag_dec_mask(index))
			}
			
			#[inline(always)]
			fn insert_diag_dec(&mut self, index: usize, bits: <Self as bitboard::Bitboard>::Storage) {
				let mask = Self::compute_diag_dec_mask(index);
				let new = mask.pdep(bits);
				let cleared = Self::from_storage(self.storage() & !mask.storage());
				*self = cleared | new;
			}
			fn generate_attack_tables_pext(
				mask_fn: fn(u8) -> Self,
				attack_fn: fn(u8, Self) -> Self
			) -> Vec<Vec<Self>>
			{
				let mut tables = Vec::new();

				for sq in 0..Self::NB_SQUARES as u8 {
					let mask = mask_fn(sq);
					let bits = mask.count() as u8;
					let table_size = 1usize << bits;

					let mut table = vec![Self::empty(); table_size];

					for index in 0..table_size {
						let blockers = mask.pdep(<<Self as bitboard::Bitboard>::Storage as bitboard::IntegerStorage>::from_usize(index));
						let attacks = attack_fn(sq, blockers);
						table[index] = attacks;
					}

					tables.push(table);
				}

				tables
			}
		}
		impl #struct_ident {
			#[inline(always)]
			pub const fn row_mask(y: u8) -> Self {
				if Self::COL_MAJOR {
					Self(Self::WEST_BORDER.0 << y)
				} else {
					Self((((1 as #storage_ty) << Self::WIDTH as usize) - 1) << (y as usize * Self::WIDTH as usize))
				}
			}
			#[inline(always)]
			pub const fn col_mask(x: u8) -> Self {
				if Self::COL_MAJOR {
					Self((((1 as #storage_ty) << Self::HEIGHT as usize) - 1) << (x as usize * Self::HEIGHT as usize))
				} else {
					Self(Self::WEST_BORDER.0 << x)
				}
			}

			#[inline]
			pub const fn compute_neighbors_ortho_mask(index: usize) -> Self {
				let (x, y) = Self::coords_from_index(index);
				let mut bb = Self(0);

				if x > 0 {
					bb = Self::from_storage(bb.storage() | Self::from_index(Self::index_from_coords(x - 1, y)).storage());
				}
				if x + 1 < Self::WIDTH {
					bb = Self::from_storage(bb.storage() | Self::from_index(Self::index_from_coords(x + 1, y)).storage());
				}
				if y > 0 {
					bb = Self::from_storage(bb.storage() | Self::from_index(Self::index_from_coords(x, y - 1)).storage());
				}
				if y + 1 < Self::HEIGHT {
					bb = Self::from_storage(bb.storage() | Self::from_index(Self::index_from_coords(x, y + 1)).storage());
				}

				bb
			}

			pub const fn generate_neighbors_ortho_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self(0); Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_neighbors_ortho_mask(i);
					i += 1;
				}
				arr
			}

			#[inline]
			pub const fn compute_neighbors_diag_mask(index: usize) -> Self {
				let (x, y) = Self::coords_from_index(index);
				let mut bb = Self(0);

				// NW
				if x > 0 && y + 1 < Self::HEIGHT {
					let idx = Self::index_from_coords(x - 1, y + 1);
					bb = Self::from_storage(bb.storage() | Self::from_index(idx).storage());
				}

				// NE
				if x + 1 < Self::WIDTH && y + 1 < Self::HEIGHT {
					let idx = Self::index_from_coords(x + 1, y + 1);
					bb = Self::from_storage(bb.storage() | Self::from_index(idx).storage());
				}

				// SW
				if x > 0 && y > 0 {
					let idx = Self::index_from_coords(x - 1, y - 1);
					bb = Self::from_storage(bb.storage() | Self::from_index(idx).storage());
				}

				// SE
				if x + 1 < Self::WIDTH && y > 0 {
					let idx = Self::index_from_coords(x + 1, y - 1);
					bb = Self::from_storage(bb.storage() | Self::from_index(idx).storage());
				}

				bb
			}
			pub const fn generate_neighbors_diag_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self(0); Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_neighbors_diag_mask(i);
					i += 1;
				}
				arr
			}

			#[inline]
			pub const fn compute_neighbors_8_mask(index: usize) -> Self {
				let ortho = Self::compute_neighbors_ortho_mask(index);
				let diag  = Self::compute_neighbors_diag_mask(index);
				Self::from_storage(ortho.storage() | diag.storage())
			}
			pub const fn generate_neighbors_8_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self(0); Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_neighbors_8_mask(i);
					i += 1;
				}
				arr
			}

			#[inline]
			const fn compute_ray_mask(index: usize, dx: isize, dy: isize) -> Self {
				let (mut x, mut y) = Self::coords_from_index(index);
				let mut bb = Self(0);

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
					bb = Self::from_storage(bb.storage() | Self::from_index(idx).storage());
				}

				bb
			}

			#[inline]
			pub const fn compute_ray_n_mask(index: usize) -> Self {
				Self::compute_ray_mask(index, 0, 1)
			}

			pub const fn generate_ray_n_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self(0); Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_ray_n_mask(i);
					i += 1;
				}
				arr
			}


			#[inline]
			pub const fn compute_ray_s_mask(index: usize) -> Self {
				Self::compute_ray_mask(index, 0, -1)
			}
			pub const fn generate_ray_s_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self(0); Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_ray_s_mask(i);
					i += 1;
				}
				arr
			}

			#[inline]
			pub const fn compute_ray_e_mask(index: usize) -> Self {
				Self::compute_ray_mask(index, 1, 0)
			}
			pub const fn generate_ray_e_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self(0); Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_ray_e_mask(i);
					i += 1;
				}
				arr
			}

			#[inline]
			pub const fn compute_ray_w_mask(index: usize) -> Self {
				Self::compute_ray_mask(index, -1, 0)
			}
			pub const fn generate_ray_w_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self(0); Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_ray_w_mask(i);
					i += 1;
				}
				arr
			}

			#[inline]
			pub const fn compute_ray_ne_mask(index: usize) -> Self {
				Self::compute_ray_mask(index, 1, 1)
			}
			pub const fn generate_ray_ne_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self(0); Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_ray_ne_mask(i);
					i += 1;
				}
				arr
			}

			#[inline]
			pub const fn compute_ray_nw_mask(index: usize) -> Self {
				Self::compute_ray_mask(index, -1, 1)
			}
			pub const fn generate_ray_nw_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self(0); Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_ray_nw_mask(i);
					i += 1;
				}
				arr
			}

			#[inline]
			pub const fn compute_ray_se_mask(index: usize) -> Self {
				Self::compute_ray_mask(index, 1, -1)
			}
			pub const fn generate_ray_se_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self(0); Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_ray_se_mask(i);
					i += 1;
				}
				arr
			}

			#[inline]
			pub const fn compute_ray_sw_mask(index: usize) -> Self {
				Self::compute_ray_mask(index, -1, -1)
			}
			pub const fn generate_ray_sw_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self(0); Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_ray_sw_mask(i);
					i += 1;
				}
				arr
			}

			#[inline(always)]
			pub const fn compute_diag_inc_mask(index: usize) -> Self {
				let (x0, y0) = Self::coords_from_index(index);
				let mut bb = Self(0);

				let mut x = x0+1;
				let mut y = y0+1;
				loop {
					if x >= Self::WIDTH || y >= Self::HEIGHT {
						break;
					}
					bb = Self::from_storage(bb.storage() | Self::from_index(Self::index_from_coords(x, y)).storage());

					x += 1;
					y += 1;
				}
				let mut x = x0;
				let mut y = y0;
				loop {
					bb = Self::from_storage(bb.storage() | Self::from_index(Self::index_from_coords(x, y)).storage());

					if x == 0 || y == 0 {
						break;
					}
					x -= 1;
					y -= 1;
				}
				bb
			}

			pub const fn generate_diag_inc_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self(0); Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_diag_inc_mask(i);
					i += 1;
				}
				arr
			}

			#[inline(always)]
			pub const fn compute_diag_dec_mask(index: usize) -> Self {
				let (x0, y0) = Self::coords_from_index(index);
				let mut bb = Self(0);

				let mut x = x0 as i16 + 1;
				let mut y = y0 as i16 - 1;
				loop {
					if x >= Self::WIDTH as i16 || y < 0 {
						break;
					}
					bb = Self::from_storage(bb.storage() | Self::from_index(Self::index_from_coords(x as u8, y as u8)).storage());

					x += 1;
					y -= 1;
				}
				let mut x = x0 as i16;
				let mut y = y0 as i16;
				loop {
					if x < 0 || y >= Self::HEIGHT as i16 {
						break;
					}
					bb = Self::from_storage(bb.storage() | Self::from_index(Self::index_from_coords(x as u8, y as u8)).storage());

					x -= 1;
					y += 1;
				}
				bb
			}
			pub const fn generate_diag_dec_table() -> [Self; Self::NB_SQUARES] {
				let mut arr = [Self(0); Self::NB_SQUARES];
				let mut i = 0;
				while i < Self::NB_SQUARES {
					arr[i] = Self::compute_diag_dec_mask(i);
					i += 1;
				}
				arr
			}
			
			#[inline(always)]
			pub const fn compute_row_mask(index: usize) -> Self {
				let y = if Self::COL_MAJOR {
					(index % Self::HEIGHT as usize) as u8
				} else {
					(index / Self::WIDTH as usize) as u8
				};
				Self::row_mask(y as u8)
			}

			#[inline(always)]
			pub const fn compute_col_mask(index: usize) -> Self {
				let x = if Self::COL_MAJOR {
					(index / Self::HEIGHT as usize) as u8
				} else {
					(index % Self::WIDTH as usize) as u8
				};
				Self::col_mask(x as u8)
			}

			#[inline(always)]
			pub const fn shift_scanline(&self, dx: i16, dy: i16) -> Self {
				let mut out: #storage_ty = 0;

				let w = Self::WIDTH as i16;
				let h = Self::HEIGHT as i16;

				let mut idx = 0;
				while idx < (Self::WIDTH * Self::HEIGHT) {
					if (self.0 >> idx) & 1 == 1 {
						let (x, y) = Self::coords_from_index(idx as usize);

						let nx = x as i16 + dx;
						let ny = y as i16 + dy;

						if nx >= 0 && nx < w && ny >= 0 && ny < h {
							let nidx = Self::index_from_coords(nx as u8, ny as u8);
							out |= 1 << nidx;
						}
					}
					idx += 1;
				}

				Self(out)
			}
			#[inline(always)]
			pub fn shift(&self, dx: i16, dy: i16) -> Self {
				if dx == 0 && dy == 0 {
					return *self;
				}

				let w = Self::WIDTH as i16;
				let h = Self::HEIGHT as i16;

				let mut out = Self::empty();

				if !Self::COL_MAJOR {
					let lane_mask: #storage_ty =
						((1 as #storage_ty) << Self::WIDTH) - 1;

					let mut y = 0;
					while y < Self::HEIGHT {
						let ny = y as i16 + dy;
						if ny < 0 || ny >= h {
							y += 1;
							continue;
						}

						let mut row = self.extract_row(y as u8);

						if dx > 0 {
							let s = dx as usize;
							if s as i16 >= w {
								y += 1;
								continue;
							}
							row = (row << s) & lane_mask;
						} else if dx < 0 {
							let s = (-dx) as usize;
							if s as i16 >= w {
								y += 1;
								continue;
							}
							row = (row >> s) & lane_mask;
						}

						out.insert_row(ny as u8, row);
						y += 1;
					}
				} else {
					// --- col-major
					let lane_mask: #storage_ty =
						((1 as #storage_ty) << Self::HEIGHT) - 1;

					let mut x = 0;
					while x < Self::WIDTH {
						let nx = x as i16 + dx;
						if nx < 0 || nx >= w {
							x += 1;
							continue;
						}

						let mut col = self.extract_col(x as u8);

						if dy > 0 {
							let s = dy as usize;
							if s as i16 >= h {
								x += 1;
								continue;
							}
							col = (col << s) & lane_mask;
						} else if dy < 0 {
							let s = (-dy) as usize;
							if s as i16 >= h {
								x += 1;
								continue;
							}
							col = (col >> s) & lane_mask;
						}

						out.insert_col(nx as u8, col);
						x += 1;
					}
				}

				out
			}

			#[inline(always)]
			pub const fn offset_coords_to_index(dx: i16, dy: i16) -> i16 {
				dx * Self::H_OFFSET as i16 + dy * Self::V_OFFSET as i16
			}
			#[inline(always)]
			const fn get_border_mask(offset: i16) -> <Self as bitboard::Bitboard>::Storage {
				match offset {
					o if o ==  Self::H_OFFSET as i16 => Self::NO_WRAP_E_MASK,
					o if o == -(Self::H_OFFSET as i16) => Self::NO_WRAP_W_MASK,
					o if o ==  Self::V_OFFSET as i16 => Self::NO_WRAP_N_MASK,
					o if o == -(Self::V_OFFSET as i16) => Self::NO_WRAP_S_MASK,

					o if o ==  Self::H_OFFSET as i16 + Self::V_OFFSET as i16 => Self::NO_WRAP_NE_MASK,
					o if o == -(Self::H_OFFSET as i16) + Self::V_OFFSET as i16 => Self::NO_WRAP_NW_MASK,
					o if o ==  Self::H_OFFSET as i16 - Self::V_OFFSET as i16 => Self::NO_WRAP_SE_MASK,
					o if o == -(Self::H_OFFSET as i16) - Self::V_OFFSET as i16 => Self::NO_WRAP_SW_MASK,

					_ => !0 as #storage_ty, // aucun masque
				}
			}
			#[inline(always)]
			pub const fn shift_n(&self) -> Self {
				// shift vers le haut = +V_OFFSET
				Self::from_storage((self.0 & Self::NO_WRAP_N_MASK) << Self::V_OFFSET)
			}

			#[inline(always)]
			pub const fn shift_s(&self) -> Self {
				// shift vers le bas = -V_OFFSET
				Self::from_storage((self.0 & Self::NO_WRAP_S_MASK) >> Self::V_OFFSET)
			}

			#[inline(always)]
			pub const fn shift_e(&self) -> Self {
				// shift vers la droite = +H_OFFSET
				Self::from_storage((self.0 & Self::NO_WRAP_E_MASK) << Self::H_OFFSET)
			}

			#[inline(always)]
			pub const fn shift_w(&self) -> Self {
				// shift vers la gauche = -H_OFFSET
				Self::from_storage((self.0 & Self::NO_WRAP_W_MASK) >> Self::H_OFFSET)
			}
			#[inline(always)]
			pub const fn shift_ne(&self) -> Self {
				let delta = Self::H_OFFSET as isize + Self::V_OFFSET as isize;
				Self::from_storage((self.0 & Self::NO_WRAP_NE_MASK) << delta)
			}

			#[inline(always)]
			pub const fn shift_nw(&self) -> Self {
				let delta = Self::V_OFFSET as isize - Self::H_OFFSET as isize;
				if delta >= 0 {
					Self::from_storage((self.0 & Self::NO_WRAP_NW_MASK) << delta)
				} else {
					Self::from_storage((self.0 & Self::NO_WRAP_NW_MASK) >> (-delta))
				}
			}

			#[inline(always)]
			pub const fn shift_se(&self) -> Self {
				let delta = Self::H_OFFSET as isize - Self::V_OFFSET as isize;
				if delta >= 0 {
					Self::from_storage((self.0 & Self::NO_WRAP_SE_MASK) << delta)
				} else {
					Self::from_storage((self.0 & Self::NO_WRAP_SE_MASK) >> (-delta))
				}
			}

			#[inline(always)]
			pub const fn shift_sw(&self) -> Self {
				let delta = -(Self::H_OFFSET as isize + Self::V_OFFSET as isize);
				Self::from_storage((self.0 & Self::NO_WRAP_SW_MASK) >> (-delta))
			}


			pub const NO_WRAP_N_MASK : #storage_ty = !Self::row_mask(Self::HEIGHT - 1).storage();
			pub const NO_WRAP_S_MASK : #storage_ty = !Self::row_mask(0).storage();
			pub const NO_WRAP_E_MASK : #storage_ty = !Self::col_mask(Self::WIDTH - 1).storage();
			pub const NO_WRAP_W_MASK : #storage_ty = !Self::col_mask(0).storage();
			pub const NO_WRAP_NE_MASK : #storage_ty = Self::NO_WRAP_N_MASK & Self::NO_WRAP_E_MASK;
			pub const NO_WRAP_NW_MASK : #storage_ty = Self::NO_WRAP_N_MASK & Self::NO_WRAP_W_MASK;
			pub const NO_WRAP_SE_MASK : #storage_ty = Self::NO_WRAP_S_MASK & Self::NO_WRAP_E_MASK;
			pub const NO_WRAP_SW_MASK : #storage_ty = Self::NO_WRAP_S_MASK & Self::NO_WRAP_W_MASK;
			
			
		}
		#derive_bitboard_mask_res
		impl Copy for #struct_ident {}
		impl Clone for #struct_ident {
			fn clone(&self) -> Self {
				Self(self.0)
			}
		}
		impl PartialEq for #struct_ident {
			fn eq(&self, other: &Self) -> bool {
				self.0 == other.0
			}
		}
		impl Eq for #struct_ident {
		}
	};
	let impl_array = quote! {
		impl bitboard::Bitboard for #struct_ident {
			type Storage = #storage_ty;
			fn width(&self) -> u8 {
				Self::WIDTH
			}
			fn height(&self) -> u8 {
				Self::HEIGHT
			}
			fn col_major(&self) -> bool {
				Self::COL_MAJOR
			}
			#[inline]
			fn is_empty(&self) -> bool {
				for a in self.0.iter() {
					if *a != 0 {
						return false;
					}
				}
				true
			}
			#[inline]
			fn count(&self) -> u32 {
				let mut counts=0;
				for a in self.0.iter() {
					counts+=a.count_ones();
				}
				counts
			}
			#[inline]
			fn intersects(&self, other: &Self) -> bool {
				for (a, b) in self.0.iter().zip(other.0.iter()) {
					if *a & *b != 0 {
						return false;
					}
				}
				true
			}
			#[inline]
			fn storage(&self) -> &Self::Storage {
				&self.0
			}
			#[inline]
			fn storage_mut(&mut self) -> &mut Self::Storage {
				&mut self.0
			}
			#[inline]
			fn get_at_index(&self, idx: usize) -> bool {
				let byte = idx / 64;
				let bit = idx % 64;
				(self.0[byte] >> bit) & 1 == 1
			}
			
			#[inline]
			fn set_value_at_index(&mut self, idx: usize, val: bool) {
				let byte = idx / 64;
				let bit = idx % 64;
				if val {
					self.0[byte] |= 1 << bit;
				} else {
					self.0[byte] &= !(1 << bit);
				}
			}
			#[inline]
			fn set_at_index(&mut self, idx: usize) {
				let byte = idx / 64;
				let bit = idx % 64;
				self.0[byte] |= 1 << bit;
			}
			#[inline]
			fn reset_at_index(&mut self, idx: usize) {
				let byte = idx / 64;
				let bit = idx % 64;
				self.0[byte] &= !(1 << bit);
			}
			#[inline]
			fn toggle_at_index(&mut self, idx: usize) {
				let byte = idx / 64;
				let bit = idx % 64;
				self.0[byte] ^= 1 << bit;
			}
			#[inline(always)]
			fn get(&self, x: u8, y: u8) -> bool {
				let idx = Self::index_from_coords(x, y);
				self.get_at_index(idx)
			}
			/// Sets the bit at coordinates `(x, y)`.
			#[inline(always)]
			fn set_value(&mut self, x: u8, y: u8, val: bool) {
				let idx = Self::index_from_coords(x, y);
				self.set_value_at_index(idx, val)
			}
			#[inline(always)]
			fn set(&mut self, x: u8, y: u8) {
				let idx = Self::index_from_coords(x, y);
				self.set_at_index(idx)
			}
			#[inline(always)]
			fn reset(&mut self, x: u8, y: u8) {
				let idx = Self::index_from_coords(x, y);
				self.reset_at_index(idx)
			}
			#[inline(always)]
			fn flipped(&self) -> Self {
				let mut a = self.0.clone();
				for b in a.iter_mut() {
					*b=!(*b);
				}
				Self(a)
			}
			#[inline(always)]
			fn lsb(&self) -> u32 {
				self.0[0].trailing_zeros()
			}
			fn pop_lsb(&mut self) -> u32 {
				for (word_index, word) in self.0.iter_mut().enumerate() {
					if *word != 0 {
						let lsb = word.trailing_zeros();
						*word &= *word - 1; // clear lowest set bit
						return (word_index as u32) * 64 + lsb;
					}
				}
				u32::MAX // no bit found
			}
			
			fn pext(&self, mask: &Self) -> Self::Storage {
				let mut out = [0;#array_bytes];
				let mut current: u64 = 0;
				let mut bitpos = 0;
				let mut out_index = 0;
				
				for (a, m) in self.0.iter().zip(mask.0.iter()) {
					let mut mm = *m;
					while mm != 0 {
						let lsb = mm.trailing_zeros();
						let bit = (a >> lsb) & 1;
						
						current |= bit << bitpos;
						bitpos += 1;
						
						if bitpos == 64 {
							out[out_index] = current;
							current = 0;
							bitpos = 0;
						}
						
						mm &= mm - 1; // clear extracted bit
					}
				}
				
				if bitpos > 0 {
					out[out_index] = current;
				}
				
				out
			}
			
			fn pdep(&self, compressed: Self::Storage) -> Self {
				let mut out = [0;#array_bytes];
				
				let mut src_word_index = 0;
				let mut src_bit_index = 0;
				
				for (out_word, mask_word) in out.iter_mut().zip(self.0.iter()) {
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
				
				Self(out)
			}
			#[inline(always)]
			fn extract_row(&self, y: u8) -> Self::Storage {
				let mask = Self::row_mask(y);
				self.pext(&mask)
			}
			#[inline(always)]
			fn extract_col(&self, x: u8) -> Self::Storage {
				let mask = Self::col_mask(x);
				self.pext(&mask)
			}
			#[inline(always)]
			fn insert_row(&mut self, y: u8, row_bits: Self::Storage) {
				let mask = Self::row_mask(y);
				
				let new_row = mask.pdep(row_bits);
				//clear
				*self &= !mask;
				// insert
				*self |= new_row;
			}
			
			#[inline(always)]
			fn insert_col(&mut self, x: u8, col_bits: Self::Storage) {
				let mask = Self::col_mask(x);
				
				let new_row = mask.pdep(col_bits);
				//clear
				*self &= !mask;
				// insert
				*self |= new_row;
			}
		}
		impl #struct_ident {

			#bitboard_impl_common
			pub const EMPTY: Self = Self([0;#array_bytes]);
			pub const FULL: Self = Self([0xFFFFFFFF;#array_bytes]);
			#[inline]
			pub const fn new() -> Self {
				Self::empty()
			}
			#[inline]
			pub const fn empty() -> Self {
				Self([0;#array_bytes])
			}
			
			/// Construct a Bitboard from its storage representation
			#[inline]
			pub const fn from_storage(v: <Self as bitboard::Bitboard>::Storage) -> Self {
				Self(v)
			}
			/// Construct a Bitboard with a single bit set at (x, y)
			#[inline]
			pub const fn from_coords(x: u8, y: u8) -> Self {
				let mut inst = Self::new();
				inst.set(x, y);
				inst
			}
			/// Construct a Bitboard with a single bit set at specified index
			#[inline]
			pub const fn from_index(index: usize) -> Self {
				let mut inst = Self::new();
				inst.set_at_index(index);
				inst
			}
			#[inline(always)]
			pub const fn storage(&self) -> <Self as bitboard::Bitboard>::Storage {
				self.0
			}
			#[inline]
			pub const fn get_at_index(&self, idx: usize) -> bool {
				let byte = idx / 64;
				let bit = idx % 64;
				(self.0[byte] >> bit) & 1 == 1
			}
			
			#[inline]
			pub const fn set_value_at_index(&mut self, idx: usize, val: bool) {
				let byte = idx / 64;
				let bit = idx % 64;
				if val {
					self.0[byte] |= 1 << bit;
				} else {
					self.0[byte] &= !(1 << bit);
				}
			}
			#[inline]
			pub const fn set_at_index(&mut self, idx: usize) {
				let byte = idx / 64;
				let bit = idx % 64;
				self.0[byte] |= 1 << bit;
			}
			#[inline]
			pub const fn reset_at_index(&mut self, idx: usize) {
				let byte = idx / 64;
				let bit = idx % 64;
				self.0[byte] &= !(1 << bit);
			}
			#[inline]
			pub const fn toggle_at_index(&mut self, idx: usize) {
				let byte = idx / 64;
				let bit = idx % 64;
				self.0[byte] ^= 1 << bit;
			}
			#[inline(always)]
			pub const fn get(&self, x: u8, y: u8) -> bool {
				let idx = Self::index_from_coords(x, y);
				self.get_at_index(idx)
			}
			/// Sets the bit at coordinates `(x, y)`.
			#[inline(always)]
			pub const fn set_value(&mut self, x: u8, y: u8, val: bool) {
				let idx = Self::index_from_coords(x, y);
				self.set_value_at_index(idx, val)
			}
			#[inline(always)]
			pub const fn set(&mut self, x: u8, y: u8) {
				let idx = Self::index_from_coords(x, y);
				self.set_at_index(idx)
			}
			#[inline(always)]
			pub const fn reset(&mut self, x: u8, y: u8) {
				let idx = Self::index_from_coords(x, y);
				self.reset_at_index(idx)
			}
			/// Get the bit index from (x, y)
			#[inline]
			pub const fn index_from_coords(x: u8, y: u8) -> usize {
				if Self::COL_MAJOR {
					x as usize * Self::HEIGHT as usize + y as usize
				} else {
					y as usize * Self::WIDTH as usize + x as usize
				}
			}

			/// Get the (x, y) coords from bit index 
			#[inline]
			pub const fn coords_from_index(i: usize) -> (u8, u8) {
				if Self::COL_MAJOR {
					((i / Self::HEIGHT as usize) as u8, (i % Self::HEIGHT as usize) as u8)
				} else {
					((i % Self::WIDTH as usize) as u8, (i / Self::WIDTH as usize) as u8)
				}
			}

			/// Check (x, y) is inside the bitboard
			#[inline]
			pub const fn is_in_bounds(x: u8, y: u8) -> bool {
				x < Self::WIDTH && y < Self::HEIGHT
			}
			/// Check index is inside the bitboard
			#[inline]
			pub const fn is_index_in_bounds(i: usize) -> bool {
				i < Self::WIDTH as usize * Self::HEIGHT as usize
			}

			#[inline(always)]
			pub fn row_mask(y: u8) -> Self {
				if Self::COL_MAJOR {
					//TODO: WEST_BORDER compute
					let nb_bits = Self::WIDTH as usize * Self::HEIGHT as usize;
					let nb_words = (nb_bits + 63) / 64;
					
					let mut bits = [0;#array_bytes];
					
					for y in 0..Self::HEIGHT {
						let idx = y as usize * Self::WIDTH as usize;
						let word = idx / 64;
						let bit  = idx % 64;
						
						bits[word] |= 1u64 << bit;
					}
					Self::from_storage(bits) << y
				} else {
					let mut row = Self::empty();
					row.0[0]=1;
					row <<= Self::WIDTH as usize;
					row -= 1;
					row <<= y as usize * Self::WIDTH as usize;
					row
				}
			}
			#[inline(always)]
			pub fn col_mask(x: u8) -> Self {
				if Self::COL_MAJOR {
					let mut col = Self::empty();
					col.0[0]=1;
					col <<= Self::HEIGHT as usize;
					col -= 1;
					col <<= x as usize * Self::HEIGHT as usize;
					col
				} else {
					//TODO: WEST_BORDER compute
					let nb_bits = Self::WIDTH as usize * Self::HEIGHT as usize;
					let nb_words = (nb_bits + 63) / 64;
					
					let mut bits = [0;#array_bytes];
					
					for y in 0..Self::HEIGHT {
						let idx = y as usize * Self::WIDTH as usize;
						let word = idx / 64;
						let bit  = idx % 64;
						
						bits[word] |= 1u64 << bit;
					}
					Self::from_storage(bits) << x
				}
			}
		}
		impl Clone for #struct_ident {
			fn clone(&self) -> Self {
				Self(self.0)
			}
		}
		impl PartialEq for #struct_ident {
			fn eq(&self, other: &Self) -> bool {
				self.0 == other.0
			}
		}
		impl Eq for #struct_ident {
		}
	};
	let is_array = total_bits > 128;
	
	let expanded_impl = if is_array {
		impl_array
	} else {
		impl_int
	};
	
	let output = quote! {
		#(#struct_attrs)*
		#struct_vis
		#expanded_struct
		#expanded_impl
		//#impl_alignments
	};
	
	output.into()
}

//fn flip(&mut self) {
//    for b in &mut self.0 {
//        *b = !*b;
//    }
//    let excess = self.0.len() * 8 - #total_bits;
//    if excess > 0 {
//        let mask = 0xFFu8 >> excess;
//        let last = self.0.len() - 1;
//        self.0[last] &= mask;
//    }
//}
//
//fn mask_flip(&mut self, mask: Self) {
//    for (a, b) in self.0.iter_mut().zip(mask.0.iter()) {
//        *a ^= *b;
//    }
//}
/*let h_offset = 1;
let v_offset = width;
let diag_inc_offset = width - 1;
let diag_dec_offset = width + 1;
let h_mask = !(left_mask | right_mask);
let v_mask = !(top_mask | bottom_mask);
let diag_inc_mask = !(left_mask | top_mask);
let diag_dec_mask = !(right_mask | top_mask);
let storage_ty = syn::parse2(storage_ty).unwrap();
let align_h = generate_alignment_fn("has_alignment_h", h_offset, &storage_ty, 7);
let align_v = generate_alignment_fn("has_alignment_v", v_offset, &storage_ty, 7);
let align_diag_inc = generate_alignment_fn("has_alignment_diag_inc", diag_inc_offset, &storage_ty, 7);
let align_diag_dec = generate_alignment_fn("has_alignment_diag_dec", diag_dec_offset, &storage_ty, 7);
let impl_alignments = quote! {
impl #struct_ident {
#align_h
#align_v
#align_diag_inc
#align_diag_dec

pub const fn has_alignment<const N: usize>(&self) -> bool {
self.has_alignment_h::<N>()
|| self.has_alignment_v::<N>()
|| self.has_alignment_diag_inc::<N>()
|| self.has_alignment_diag_dec::<N>()
}
}
};*/
#[proc_macro_derive(BitboardMask)]
pub fn derive_bitboard_mask(item: TokenStream) -> TokenStream {
	use syn::{parse_macro_input, ItemStruct, Fields};
	
	let input = parse_macro_input!(item as ItemStruct);
	let ident = input.ident.clone();
	
	let storage_ty = match &input.fields {
		Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
			fields.unnamed.first().unwrap().ty.clone()
		}
		_ => {
			return syn::Error::new_spanned(
				input,
				"BitboardMask can only be derived on tuple structs with one field"
			)
			.to_compile_error()
			.into();
		}
	};
	
	let is_array = matches!(&storage_ty, syn::Type::Array(_));
	
	let impl_int = bitboard_mask_int_impl(&ident);
	
	let impl_array = quote! {
		impl std::ops::BitAnd for #ident {
			type Output = Self;
			#[inline(always)]
			fn bitand(self, rhs: Self) -> Self {
				let mut out = self.0;
				for (a, b) in out.iter_mut().zip(rhs.0.iter()) {
					*a &= *b;
				}
				Self(out)
			}
		}
		
		impl std::ops::BitOr for #ident {
			type Output = Self;
			#[inline(always)]
			fn bitor(self, rhs: Self) -> Self {
				let mut out = self.0;
				for (a, b) in out.iter_mut().zip(rhs.0.iter()) {
					*a |= *b;
				}
				Self(out)
			}
		}
		
		impl std::ops::BitXor for #ident {
			type Output = Self;
			#[inline(always)]
			fn bitxor(self, rhs: Self) -> Self {
				let mut out = self.0;
				for (a, b) in out.iter_mut().zip(rhs.0.iter()) {
					*a ^= *b;
				}
				Self(out)
			}
		}
		
		impl std::ops::BitAndAssign for #ident {
			#[inline(always)]
			fn bitand_assign(&mut self, rhs: Self) {
				for (a, b) in self.0.iter_mut().zip(rhs.0.iter()) {
					*a &= *b;
				}
			}
		}
		
		impl std::ops::BitOrAssign for #ident {
			#[inline(always)]
			fn bitor_assign(&mut self, rhs: Self) {
				for (a, b) in self.0.iter_mut().zip(rhs.0.iter()) {
					*a |= *b;
				}
			}
		}
		
		impl std::ops::BitXorAssign for #ident {
			#[inline(always)]
			fn bitxor_assign(&mut self, rhs: Self) {
				for (a, b) in self.0.iter_mut().zip(rhs.0.iter()) {
					*a ^= *b;
				}
			}
		}
		impl std::ops::Not for #ident {
			type Output = Self;
			
			#[inline(always)]
			fn not(self) -> Self::Output {
				let mut out = self.0;
				for a in out.iter_mut() {
					*a = !*a;
				}
				Self(out)
			}
		}
		
		impl std::ops::Shl<usize> for #ident {
			type Output = Self;
			
			#[inline(always)]
			fn shl(self, rhs: usize) -> Self {
				let mut out = self.0;
				for a in out.iter_mut() {
					*a <<= rhs;
				}
				Self(out)
			}
		}
		
		impl std::ops::Shr<usize> for #ident {
			type Output = Self;
			
			#[inline(always)]
			fn shr(self, rhs: usize) -> Self {
				let mut out = self.0;
				for a in out.iter_mut() {
					*a >>= rhs;
				}
				Self(out)
			}
		}
		impl std::ops::ShlAssign<usize> for #ident {
			#[inline(always)]
			fn shl_assign(&mut self, rhs: usize) {
				for a in self.0.iter_mut() {
					*a <<= rhs;
				}
			}
		}
		
		impl std::ops::ShrAssign<usize> for #ident {
			#[inline(always)]
			fn shr_assign(&mut self, rhs: usize) {
				for a in self.0.iter_mut() {
					*a >>= rhs;
				}
			}
		}
		impl std::ops::Shl<u8> for #ident {
			type Output = Self;
			
			#[inline(always)]
			fn shl(self, rhs: u8) -> Self {
				let mut out = self.0;
				for a in out.iter_mut() {
					*a <<= rhs;
				}
				Self(out)
			}
		}
		
		impl std::ops::Shr<u8> for #ident {
			type Output = Self;
			
			#[inline(always)]
			fn shr(self, rhs: u8) -> Self {
				let mut out = self.0;
				for a in out.iter_mut() {
					*a >>= rhs;
				}
				Self(out)
			}
		}
		
		
		impl std::ops::SubAssign<usize> for #ident {
			#[inline(always)]
			fn sub_assign(&mut self, rhs: usize) {
				let mut carry = rhs as u64;
				
				for word in self.0.iter_mut() {
					let (new_word, overflow) = word.overflowing_sub(carry);
					*word = new_word;
					carry = if overflow { 1 } else { 0 };
					
					if carry == 0 {
						break;
					}
				}
			}
		}
		impl std::ops::Sub<usize> for #ident {
			type Output = Self;
			
			fn sub(mut self, rhs: usize) -> Self::Output {
				self -= rhs as usize;
				self
			}
		}
		
	};
	
	let impl_storage = if is_array { impl_array } else { impl_int };
	
	let output = quote! {
		#impl_storage
	};
	
	output.into()
}

fn bitboard_mask_int_impl(ident: &syn::Ident) -> proc_macro2::TokenStream {
	let impl_int = quote! {
		impl std::ops::BitAnd for #ident {
			type Output = Self;
			#[inline(always)]
			fn bitand(self, rhs: Self) -> Self {
				Self(self.0 & rhs.0)
			}
		}
		
		impl std::ops::BitOr for #ident {
			type Output = Self;
			#[inline(always)]
			fn bitor(self, rhs: Self) -> Self {
				Self(self.0 | rhs.0)
			}
		}
		
		impl std::ops::BitXor for #ident {
			type Output = Self;
			#[inline(always)]
			fn bitxor(self, rhs: Self) -> Self {
				Self(self.0 ^ rhs.0)
			}
		}
		
		impl std::ops::BitAndAssign for #ident {
			#[inline(always)]
			fn bitand_assign(&mut self, rhs: Self) {
				self.0 &= rhs.0;
			}
		}
		
		impl std::ops::BitOrAssign for #ident {
			#[inline(always)]
			fn bitor_assign(&mut self, rhs: Self) {
				self.0 |= rhs.0;
			}
		}
		
		impl std::ops::BitXorAssign for #ident {
			#[inline(always)]
			fn bitxor_assign(&mut self, rhs: Self) {
				self.0 ^= rhs.0;
			}
		}
		
		impl std::ops::Not for #ident {
			type Output = Self;
			
			#[inline(always)]
			fn not(self) -> Self::Output {
				Self::from_storage(!self.storage())
			}
		}
		
		impl std::ops::Shl<usize> for #ident {
			type Output = Self;
			
			#[inline(always)]
			fn shl(self, rhs: usize) -> Self {
				Self::from_storage(self.storage() << rhs)
			}
		}
		
		impl std::ops::Shr<usize> for #ident {
			type Output = Self;
			
			#[inline(always)]
			fn shr(self, rhs: usize) -> Self {
				Self::from_storage(self.storage() >> rhs)
			}
		}
		impl std::ops::ShlAssign<usize> for #ident {
			#[inline(always)]
			fn shl_assign(&mut self, rhs: usize) {
				self.0 <<= rhs;
			}
		}
		impl std::ops::ShrAssign<usize> for #ident {
			#[inline(always)]
			fn shr_assign(&mut self, rhs: usize) {
				self.0 >>= rhs;
			}
		}
		impl std::ops::Shl<u8> for #ident {
			type Output = Self;
			
			#[inline(always)]
			fn shl(self, rhs: u8) -> Self {
				Self::from_storage(self.storage() << rhs)
			}
		}
		
		impl std::ops::Shr<u8> for #ident {
			type Output = Self;
			
			#[inline(always)]
			fn shr(self, rhs: u8) -> Self {
				Self::from_storage(self.storage() >> rhs)
			}
		}
		
	};
	impl_int
}

#[proc_macro_derive(BitboardDisplay)]
pub fn derive_bitboard_display(item: TokenStream) -> TokenStream {
	use syn::{parse_macro_input, ItemStruct, Fields};
	
	let input = parse_macro_input!(item as ItemStruct);
	let ident = input.ident.clone();
	
	match &input.fields {
		Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {}
		_ => {
			return syn::Error::new_spanned(
				input,
				"BitboardDisplay can only be derived on tuple structs with one field"
			)
			.to_compile_error()
			.into();
		}
	}
	
	let output = quote! {
		
		impl std::fmt::Display for #ident {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				::bitboard::fmt_bitboard_display(self, f)
			}
		}
		
	};
	
	output.into()
}

#[proc_macro_derive(BitboardDebug)]
pub fn derive_bitboard_debug(item: TokenStream) -> TokenStream {
	use syn::{parse_macro_input, ItemStruct, Fields};
	
	let input = parse_macro_input!(item as ItemStruct);
	let ident = input.ident.clone();
	
	match &input.fields {
		Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {}
		_ => {
			return syn::Error::new_spanned(
				input,
				"BitboardDebug can only be derived on tuple structs with one field"
			)
			.to_compile_error()
			.into();
		}
	}
	
	let output = quote! {
		impl std::fmt::Debug for #ident {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				::bitboard::fmt_bitboard_debug(self, std::mem::size_of_val(&self.0) * 8, f)
			}
		}
	};
	
	output.into()
}



/*
fn generate_alignment_fn(
name: &str,
offset: usize,
storage_ty: &syn::Type,
max_n: usize,
) -> proc_macro2::TokenStream {
let mut match_arms = Vec::new();

for n in 2..=max_n {
let mut lines = Vec::new();
let mut prev = quote! { bits };

for i in 1..n {
let ident = syn::Ident::new(&format!("x{}", i), proc_macro2::Span::call_site());
lines.push(quote! {
let #ident = #prev & (#prev >> #offset);
});
prev = quote! { #ident };
}

match_arms.push(quote! {
#n => {
#(#lines)*
#prev != 0
}
});
}

quote! {
pub const fn #name<const N: usize>(&self) -> bool {
let bits: #storage_ty = self.0;
match N {
#(#match_arms,)*
_ => false,
}
}
}
}
*/

