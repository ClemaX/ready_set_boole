use itertools::Itertools;
use bitvec::prelude::*;

type Binop = fn(u32, u32) -> u32;
type Unop = fn(u32) -> u32;

fn parser(input: &str) -> Option<bool> {
	let mut bits = bitvec![];
	let mut chars = input.chars().peekable();
	let operands = chars.peeking_take_while(|c| *c == '0' || *c == '1');

	bits.extend(operands.map(|c| c != '0'));

	for op in chars {
		let b = bits.pop().expect("missing first operand!");

		match op {
			'!' => { bits.push(b ^ true); },
			_ => {
				let a = bits.pop().expect("missing second operand!");

				match op {
					'&' => { bits.push(a & b); },
					'|' => { bits.push(a | b); },
					'^' => { bits.push(a ^ b); },
					'>' => { bits.push(a ^ true | b); },
					'=' => { bits.push(a ^ true ^ b); },
					_ => { panic!("unknown operator {:?}", op); },
				};
			},
		};
	}

	bits.pop()
}

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

fn print_rpn(input: &str) {
	let result = parser(input);

	if let Some(value) = result {
		println!("{} = {}", input, value);
	}
}

fn main() {
	print_binop(adder, 42, 101, '+');
	print_binop(multiplier, 42, 101, '*');
	print_unop(gray_code, 42, "gray");
	print_rpn("01&");
	print_rpn("01|");
	print_rpn("0!");
	print_rpn("10!&");
	print_rpn("00>");
	print_rpn("10>");
	print_rpn("110!&>");
	print_rpn("110&>");
}
