pub type Binop = fn(u32, u32) -> u32;

pub fn adder(a: u32, b: u32) -> u32 {
	let mut a = a;
	let mut b = b;
	let mut carry;

	loop {
		carry = (a & b) << 1;

		a ^= b;
		b = a & carry;
		a |= carry;

		if carry == 0 {
			break;
		}
	}

	a
}

pub fn multiplier(a: u32, b: u32) -> u32 {
	let mut result = 0;
	let mut b = b;

	while b != 0 {
		result = adder(result, a);
		b -= 1;
	}

	result
}
