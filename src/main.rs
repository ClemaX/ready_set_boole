#![feature(extract_if)]
use bitvec::prelude::*;

type Binop = fn(u32, u32) -> u32;
type Unop = fn(u32) -> u32;

fn eval_formula(input: &str) -> bool {
	let mut bits = bitvec![];
	let mut chars: Vec<char> = input.chars().collect();
	let operands = chars.extract_if(|c| *c == '0' || *c == '1');

	bits.extend(operands.map(|c| c != '0'));

	//dbg!(&bits);

	for op in chars {
		let b = bits.pop().expect("missing first operand!");

		//dbg!(op);
		//dbg!(b);

		match op {
			'!' => { bits.push(b ^ true); },
			_ => {
				let a = bits.pop().expect("missing second operand!");
				//dbg!(a);

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

	bits.pop().expect("missing operation")
}

fn print_truth_row<'a>(variables: impl Iterator<Item = &'a char>, value: char) {
	for variable in variables {
		print!("| {} ", variable);
	}
	println!("| {} |", value);
}

fn print_truth_table(formula: &str) {
	let mut chars: Vec<char> = formula.to_ascii_uppercase().chars().collect();
	let variables: Vec<char> = chars.extract_if(|c| c.is_ascii_alphabetic()).collect();

	let mut unique: Vec<char> = variables.clone();
	unique.sort();
	unique.dedup();

	let variable_count = unique.len();

	let mut rpn = String::from_iter(variables.iter());
	rpn.extend(&chars);

	dbg!(&rpn);
	print_truth_row(unique.iter(), '=');

	for mut combination in 0..1 << variable_count {
		let mut substituted = rpn.clone();

		for variable in unique.iter() {
			let value = char::from_digit(combination & 1, 2).unwrap();
			print!("| {} ", value);
			substituted = substituted.replace(&variable.to_string(), &value.to_string());
			combination >>= 1;
		}
		println!("| {} |", eval_formula(&substituted) as i32);
	}
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

fn print_statement_result<T: std::fmt::Display>(statement: &str, result: T) {
	println!("{:<16} = {}", statement, result);
}

fn print_binop(op: Binop, a: u32, b: u32, symbol: char) {
	let statement = format!("{} {} {}", a, symbol, b);
	let result = op(a, b);

	print_statement_result(&statement, result);
}

fn print_unop(op: Unop, a: u32, name: &str) {
	let statement = format!("{}({})", name, a);
	let result = op(a);

	print_statement_result(&statement, result);
}

fn print_formula(input: &str) {
	
	print_statement_result(input, eval_formula(input));
}

fn main() {
	println!("Binary operations:");
	println!("=================");
	println!();
	print_binop(adder, 42, 101, '+');
	print_binop(multiplier, 42, 101, '*');
	println!();

	println!("Unary operations:");
	println!("================");
	println!();
	print_unop(gray_code, 42, "gray");
	println!();

	println!("Formulas:");
	println!("========");
	println!();
	print_formula("01&");
	print_formula("01|");
	print_formula("0!");
	print_formula("10|1&");
	print_formula("101|&");
	print_formula("00>");
	print_formula("10>");
	print_formula("110!&>");
	print_formula("110&>");
	print_formula("010&>");
	print_formula("00=");
	print_formula("10=");
	print_formula("10&1=");
	print_formula("10|1=");
	print_formula("101|&");
	println!();

	println!("Truth tables:");
	println!("============");
	println!();
	print_truth_table("AB&");
	print_truth_table("AB&C|");
	print_truth_table("AB|C&");
	print_truth_table("AB|B&");
	print_truth_table("PQ>");
}
