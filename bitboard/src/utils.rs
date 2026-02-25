pub trait TruncateFrom<T> {
	fn truncate_from(value: T) -> Self;
}
impl TruncateFrom<u128> for u128 {
	fn truncate_from(value: u128) -> Self {
		value
	}
}
impl TruncateFrom<u128> for usize {
	fn truncate_from(value: u128) -> Self {
		value as usize
	}
}
impl TruncateFrom<u128> for u64 {
	fn truncate_from(value: u128) -> Self {
		value as u64
	}
}

impl TruncateFrom<u128> for u32 {
	fn truncate_from(value: u128) -> Self {
		value as u32
	}
}

impl TruncateFrom<u128> for u16 {
	fn truncate_from(value: u128) -> Self {
		value as u16
	}
}

impl TruncateFrom<u128> for u8 {
	fn truncate_from(value: u128) -> Self {
		value as u8
	}
}
