pub type Unop = fn(u32) -> u32;

pub fn gray_code(a: u32) -> u32 {
	a ^ a >> 1
}