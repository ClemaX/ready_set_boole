#![feature(extract_if)]

use colored::Colorize;

mod binop;
mod unop;
mod rpn_formula;
mod print;

macro_rules! title {
    ($arg:tt) => {
        print!("{}\n\n", $arg.underline());
    };
}

fn exercise00() {
	title!("Exercise 00 - Adder");

	print::binop(binop::adder, 42, 101, '+');

	println!();
}

fn exercise01() {
	title!("Exercise 01 - Multiplier");

	print::binop(binop::multiplier, 42, 101, '*');

	println!();
}

fn exercise02() {
	title!("Exercise 02 - Gray code");

	print::unop(unop::gray_code, 42, "gray");

	println!();
}

fn exercise03() {
	title!("Exercise 03 - Boolean evaluation");

	print::formula("01&");
	print::formula("01|");
	print::formula("0!");
	print::formula("10|1&");
	print::formula("101|&");
	print::formula("00>");
	print::formula("10>");
	print::formula("110!&>");
	print::formula("110&>");
	print::formula("010&>");
	print::formula("00=");
	print::formula("10=");
	print::formula("10&1=");
	print::formula("10|1=");
	print::formula("101|&");
	print::formula("10|1&");
	print::formula("01|1&");

	println!();
}

fn exercise04() {
	title!("Exercise 4 - Truth table");

	print::truth_table("AB&");
	print::truth_table("AB&C|");
	print::truth_table("AB|C&");
	print::truth_table("AB|B&");
	print::truth_table("PQ>");
}

fn exercise05() {
	title!("Exercise 5 - Negation Normal Form");
	
	let root = rpn_formula::parse("AB&!");

	rpn_formula::print_tree(&root);
	/* rpn_formula::format_negation_normal("A!B!|"); */
}

fn main() {
	exercise00();
	exercise01();
	exercise02();
	exercise03();
	exercise04();

	exercise05();
}
