
use quote::quote;

pub(crate) fn bitboard_mask_array_impl(ident: &syn::Ident) -> proc_macro2::TokenStream {
	let impl_array = quote! {
		macro_rules! impl_const_bitwise {
			($name:ident, $trait:ident, $method:ident, $const_method:ident, $assign_const:ident, $trait_assign:ident, $method_assign:ident, $op_assign:tt) => {
				impl $name {
					#[inline(always)]
					pub const fn $assign_const(&mut self, rhs: &Self) {
						let mut i = 0;
						while i < self.0.len() {
							self.0[i] $op_assign rhs.0[i];
							i += 1;
						}
					}
					#[inline(always)]
					pub const fn $const_method(&self, rhs: &Self) -> Self {
						let mut i = 0;
						let mut res = [0u64; Self::ARRAY_LEN];
						while i < self.0.len() {
							res[i] = self.0[i];
							res[i] $op_assign rhs.0[i];
							i += 1;
						}
						Self(res)
					}
				}

				impl std::ops::$trait for $name {
					type Output = Self;
					#[inline(always)]
					fn $method(self, rhs: Self) -> Self {
						self.$const_method(&rhs)
					}
				}

				impl std::ops::$trait_assign for $name {
					#[inline(always)]
					fn $method_assign(&mut self, rhs: Self) {
						self.$assign_const(&rhs)
					}
				}
			};
		}

		impl_const_bitwise!(#ident, BitAnd, bitand, and_const, and_assign_const, BitAndAssign, bitand_assign, &=);
		impl_const_bitwise!(#ident, BitOr, bitor, or_const, or_assign_const, BitOrAssign, bitor_assign, |=);
		impl_const_bitwise!(#ident, BitXor, bitxor, xor_const, xor_assign_const, BitXorAssign, bitxor_assign, ^=);

		impl #ident {
			#[inline]
			pub const fn not_const(&self) -> Self {
				let mut i = 0;
				let mut res = [0u64; Self::ARRAY_LEN];
				while i < self.0.len() {
					res[i] = !self.0[i];
					i += 1;
				}
				Self(res)
			}
			#[inline]
			pub const fn not_assign_const(&mut self) {
				let mut i = 0;
				while i < self.0.len() {
					self.0[i] = !self.0[i];
					i += 1;
				}
			}
		}
		impl std::ops::Not for #ident {
			type Output = Self;
		
			#[inline(always)]
			fn not(self) -> Self::Output {
				self.not_const()
			}
		}
		impl #ident {
			#[inline]
			pub const fn shl_const(&self, rhs: usize) -> Self {
				if rhs == 0 { return Self(self.0); }
				let word_shift = rhs / u64::BITS as usize;
				let bit_shift = (rhs % u64::BITS as usize) as u32;

				if word_shift >= Self::ARRAY_LEN {
					return Self([0u64; Self::ARRAY_LEN]);
				}

				let mut res = [0u64; Self::ARRAY_LEN];
				
				if bit_shift == 0 {
					let mut i = word_shift;
					while i < Self::ARRAY_LEN {
						res[i] = self.0[i - word_shift];
						i += 1;
					}
				} else {
					let mut i = Self::ARRAY_LEN - 1;
					while i > word_shift {
						res[i] = (self.0[i - word_shift] << bit_shift) | (self.0[i - word_shift - 1] >> (u64::BITS - bit_shift));
						i -= 1;
					}
					res[word_shift] = self.0[0] << bit_shift;
				}
				Self(res)
			}
			//#[inline]
			pub const fn shl_assign_const(&mut self, rhs: usize) {
				if rhs == 0 { return; }

				let word_shift = rhs / u64::BITS as usize;
				let bit_shift = (rhs % u64::BITS as usize) as u32;

				if word_shift >= Self::ARRAY_LEN {
					*self = Self::EMPTY;
					return;
				}

				if bit_shift == 0 {
					let mut i = (Self::ARRAY_LEN - 1);
					while i >= word_shift {
						self.0[i] = self.0[i - word_shift];
						if i == word_shift { break; }
						i -= 1;
					}

					let mut j = 0;
					while j < word_shift {
						self.0[j] = 0;
						j += 1;
					}
				} else {
					let mut i = Self::ARRAY_LEN - 1;

					while i > word_shift {
						self.0[i] =
							(self.0[i - word_shift] << bit_shift)
							| (self.0[i - word_shift - 1] >> (u64::BITS - bit_shift));

						i -= 1;
					}

					self.0[word_shift] = self.0[0] << bit_shift;

					let mut j = 0;
					while j < word_shift {
						self.0[j] = 0;
						j += 1;
					}
				}
			}

			#[inline]
			pub const fn shr_const(&self, rhs: usize) -> Self {
				if rhs == 0 { return Self(self.0); }
				let word_shift = rhs / u64::BITS as usize;
				let bit_shift = (rhs % u64::BITS as usize) as u32;

				if word_shift >= Self::ARRAY_LEN {
					return Self([0u64; Self::ARRAY_LEN]);
				}

				let mut res = [0u64; Self::ARRAY_LEN];

				if bit_shift == 0 {
					let mut i = 0;
					while i < Self::ARRAY_LEN - word_shift {
						res[i] = self.0[i + word_shift];
						i += 1;
					}
				} else {
					let mut i = 0;
					while i < Self::ARRAY_LEN - word_shift - 1 {
						res[i] = (self.0[i + word_shift] >> bit_shift) | (self.0[i + word_shift + 1] << (u64::BITS - bit_shift));
						i += 1;
					}
					res[Self::ARRAY_LEN - word_shift - 1] = self.0[Self::ARRAY_LEN - 1] >> bit_shift;
				}
				Self(res)
			}
			//#[inline]
			pub const fn shr_assign_const(&mut self, rhs: usize) {
				if rhs == 0 { return; }

				let word_shift = rhs / u64::BITS as usize;
				let bit_shift = (rhs % u64::BITS as usize) as u32;

				if word_shift >= Self::ARRAY_LEN {
					*self = Self::EMPTY;
					return;
				}

				if bit_shift == 0 {
					let mut i = 0;
					while i < Self::ARRAY_LEN - word_shift {
						self.0[i] = self.0[i + word_shift];
						i += 1;
					}
				} else {
					let mut i = 0;

					while i < Self::ARRAY_LEN - word_shift - 1 {
						self.0[i] =
							(self.0[i + word_shift] >> bit_shift)
							| (self.0[i + word_shift + 1] << (u64::BITS - bit_shift));

						i += 1;
					}

					self.0[Self::ARRAY_LEN - word_shift - 1] =
						self.0[Self::ARRAY_LEN - 1] >> bit_shift;
				}

				// zero-fill
				let mut j = Self::ARRAY_LEN - word_shift;
				while j < Self::ARRAY_LEN {
					self.0[j] = 0;
					j += 1;
				}
			}
		}
		impl std::ops::Shl<usize> for #ident {
			type Output = Self;
		
			#[inline(always)]
			fn shl(self, rhs: usize) -> Self {
				self.shl_const(rhs)
			}
		}

		impl std::ops::Shl<u8> for #ident {
			type Output = Self;
			#[inline(always)]
			fn shl(self, rhs: u8) -> Self {
				self.shl_const(rhs as usize)
			}
		}

		impl std::ops::Shr<usize> for #ident {
			type Output = Self;
		
			#[inline(always)]
			fn shr(self, rhs: usize) -> Self {
				self.shr_const(rhs)
			}
		}
		impl std::ops::Shr<u8> for #ident {
			type Output = Self;
		
			#[inline(always)]
			fn shr(self, rhs: u8) -> Self {
				self.shr_const(rhs as usize)
			}
		}

		impl std::ops::ShlAssign<usize> for #ident {
			#[inline(always)]
			fn shl_assign(&mut self, rhs: usize) {
				*self = self.clone().shl_const(rhs);
			}
		}
	
		impl std::ops::ShrAssign<usize> for #ident {
			#[inline(always)]
			fn shr_assign(&mut self, rhs: usize) {
				*self = self.clone().shr_const(rhs);
			}
		}

		impl #ident {
			pub const fn sub_const(mut self, rhs: usize) -> Self {
				let mut carry = rhs as u64;
				let mut i = 0;
				while i < self.0.len() {
					let (new_word, overflow) = self.0[i].overflowing_sub(carry);
					self.0[i] = new_word;
					carry = if overflow { 1 } else { 0 };
					if carry == 0 { break; }
					i += 1;
				}
				self
			}
		}

		impl std::ops::SubAssign<usize> for #ident {
			fn sub_assign(&mut self, rhs: usize) {
				let mut carry = rhs as u64;
				for word in self.0.iter_mut() {
					let (new_word, overflow) = word.overflowing_sub(carry);
					*word = new_word;
					carry = if overflow { 1 } else { 0 };
					if carry == 0 { break; }
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
	impl_array
}

pub(crate) fn bitboard_mask_int_impl(ident: &syn::Ident) -> proc_macro2::TokenStream {
	let impl_int = quote! {
		impl #ident {
			#[inline(always)]
			pub const fn and_const(&self, rhs: &Self) -> Self { Self(self.0 & rhs.0) }
			
			#[inline(always)]
			pub const fn or_const(&self, rhs: &Self) -> Self { Self(self.0 | rhs.0) }
			
			#[inline(always)]
			pub const fn xor_const(&self, rhs: &Self) -> Self { Self(self.0 ^ rhs.0) }
			
			#[inline(always)]
			pub const fn not_const(&self) -> Self { Self(!self.0) }
			
			#[inline(always)]
			pub const fn shl_const(&self, rhs: usize) -> Self { Self(self.0 << rhs) }
			
			#[inline(always)]
			pub const fn shr_const(&self, rhs: usize) -> Self { Self(self.0 >> rhs) }

			#[inline(always)]
			pub const fn and_assign_const(&mut self, rhs: &Self) { self.0 &= rhs.0; }
			
			#[inline(always)]
			pub const fn or_assign_const(&mut self, rhs: &Self) { self.0 |= rhs.0; }
			
			#[inline(always)]
			pub const fn xor_assign_const(&mut self, rhs: &Self) { self.0 ^= rhs.0; }
			
			#[inline(always)]
			pub const fn not_assign_const(&mut self) { self.0 = !self.0; }
			
			#[inline(always)]
			pub const fn shl_assign_const(&mut self, rhs: usize) { self.0 <<= rhs; }
			
			#[inline(always)]
			pub const fn shr_assign_const(&mut self, rhs: usize) { self.0 >>= rhs; }
		}

		impl std::ops::BitAnd for #ident {
			type Output = Self;
			#[inline(always)]
			fn bitand(self, rhs: Self) -> Self { self.and_const(&rhs) }
		}
		
		impl std::ops::BitOr for #ident {
			type Output = Self;
			#[inline(always)]
			fn bitor(self, rhs: Self) -> Self { self.or_const(&rhs) }
		}
		
		impl std::ops::BitXor for #ident {
			type Output = Self;
			#[inline(always)]
			fn bitxor(self, rhs: Self) -> Self { self.xor_const(&rhs) }
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
			fn not(self) -> Self { self.not_const() }
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
