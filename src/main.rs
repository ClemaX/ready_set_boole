type Binop = fn(u32, u32) -> u32;
type Unop = fn(u32) -> u32;

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
	
	a
}

fn multiplier(a: u32, b: u32) -> u32 {
	let mut result = 0;
	let mut b = b;

	while b != 0 {
		result = adder(result, a);
		b -= 1;
	}

	result
}

fn gray_code(a: u32) -> u32 {
	a ^ a >> 1
}

fn print_binop(op: Binop, a: u32, b: u32, symbol: char) {
	println!("{} {} {} = {}", a, symbol, b, op(a, b));
}

fn print_unop(op: Unop, a: u32, name: &str) {
	println!("{}({}) = {}", name, a, op(a));
}

fn main() {
	print_binop(adder, 42, 101, '+');
	print_binop(multiplier, 42, 101, '*');
	print_unop(gray_code, 42, "gray");
}
