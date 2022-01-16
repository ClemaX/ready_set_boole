type Binop = fn(u32, u32) -> u32;

fn adder(a: u32, b: u32) -> u32 {
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
	return a;
}

fn print_op(op: Binop, a: u32, b: u32, symbol: char) {
	println!("{} {} {} = {}", a, symbol, b, op(a, b));
}

fn main() {
	print_op(adder, 42, 101, '+');
}
