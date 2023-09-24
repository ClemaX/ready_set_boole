#![feature(extract_if)]

use bitvec::prelude::*;
use colored::Colorize;

type Binop = fn(u32, u32) -> u32;
type Unop = fn(u32) -> u32;

macro_rules! title {
    ($arg:tt) => {
        print!("{}:\n\n", $arg.underline());
    };
}

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

fn print_table_row<T: std::fmt::Display>(contents: &Vec<T>, width: Option<usize>,
	left: Option<char>, middle: Option<char>, right: Option<char>) {

	let width = width.unwrap_or(3);
	let left = left.unwrap_or('│');
	let middle = middle.unwrap_or('│');
	let right = right.unwrap_or('│');

	for (i, content) in contents.iter().enumerate() {
		let c = if i == 0 { left } else { middle };
		
		print!("{}{:^width$}", c, content, width = width);
	}

	print!("{}\n", right);
}

fn print_table_sep(sep: char, column_count: usize, width: Option<usize>,
	left: Option<char>, middle: Option<char>, right: Option<char>) {
	let width = width.unwrap_or(3);
	let left = left.unwrap_or('├');
	let middle = middle.unwrap_or('┼');
	let right = right.unwrap_or('┤');

	let separator = String::from_iter(vec![sep; width].iter());

	for i in 0..column_count {
		let c = if i == 0 { left } else { middle };
		
		print!("{}{}", c, separator);
	}
	println!("{}", right);
}

fn print_table_header<T: std::fmt::Display>(label: &str,
	columns: &Vec<T>, width: Option<usize>,
	left: Option<char>, middle: Option<char>, right: Option<char>
) {
	let width = width.unwrap_or(3);
	let left = left.unwrap_or('│');
	let middle = middle.unwrap_or('│');
	let right = right.unwrap_or('│');
	let middle_sep_out = '─';

	let column_count = columns.len();
	let table_width = column_count * (width + 1) + 1;
	let content_width = table_width - 2;

	if !label.is_empty() {
		print_table_sep(middle_sep_out, column_count, Some(width),
			Some('╭'), Some(middle_sep_out), Some('╮'));
		print_table_row(&vec![label], Some(content_width),
			Some(left), Some(middle), Some(right));
		print_table_sep('═', column_count, None,
			Some('╞'), Some('╤'), Some('╡'));
		print_table_row(columns, Some(width),
			Some(left), Some(middle), Some(right));
	}

	//print_table_row(columns, '│', '│', '│');
}

fn print_truth_table(formula: &str) {
	let mut chars: Vec<char> = formula.to_ascii_uppercase().chars().collect();
	let mut variables: Vec<char> = chars.extract_if(|c| c.is_ascii_alphabetic()).collect();

	let mut unique: Vec<char> = variables.clone();
	
	let variable_count = variables.len();

	let mut values = vec![0; variable_count + 1];

	unique.sort();
	unique.dedup();

	let mut rpn = String::from_iter(variables.iter());
	rpn.extend(&chars);

	//dbg!(&rpn);
	variables.push('=');

	print_table_header(formula, &variables, None, None, None, None);

	variables.pop();


	for mut combination in 0..1 << variable_count {
		let mut substituted = rpn.clone();

		if combination == 0 {
			print_table_sep('═', variable_count + 1,
				None, Some('╞'), Some('╪'), Some('╡'));
		}
		else {
			print_table_sep('─', variable_count + 1,
				None, None, None, None);
		}

		for (i, variable) in unique.iter().enumerate() {
			let variable_value = combination & 1;

			values[i] = variable_value;

			substituted = substituted.replace(&variable.to_string(), &variable_value.to_string());
			combination >>= 1;
		}
		
		values[variable_count] = eval_formula(&substituted) as i32;

		print_table_row(&values, None, None, None, None);
	}

	print_table_sep('─', variable_count + 1,
		None, Some('╰'), Some('┴'), Some('╯'));
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
	title!("Binary operations");
	print_binop(adder, 42, 101, '+');
	print_binop(multiplier, 42, 101, '*');
	println!();

	title!("Unary operations");
	print_unop(gray_code, 42, "gray");
	println!();

	title!("Formulas");
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

	title!("Truth tables");
	print_truth_table("AB&");
	print_truth_table("AB&C|");
	print_truth_table("AB|C&");
	print_truth_table("AB|B&");
	print_truth_table("PQ>");
}
