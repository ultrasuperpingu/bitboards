
use quote::quote;

pub(crate) fn bitboard_mask_array_impl(ident: &syn::Ident) -> proc_macro2::TokenStream {
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
	impl_array
}

pub(crate) fn bitboard_mask_int_impl(ident: &syn::Ident) -> proc_macro2::TokenStream {
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
