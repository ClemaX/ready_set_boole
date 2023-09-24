use crate::rpn_formula;
use crate::binop::Binop;
use crate::unop::Unop;

pub fn statement_result<T: std::fmt::Display>(statement: &str, result: T) {
	println!("{:<16} = {}", statement, result);
}

pub fn binop(op: Binop, a: u32, b: u32, symbol: char) {
	let statement = format!("{} {} {}", a, symbol, b);
	let result = op(a, b);

	statement_result(&statement, result);
}

pub fn unop(op: Unop, a: u32, name: &str) {
	let statement = format!("{}({})", name, a);
	let result = op(a);

	statement_result(&statement, result);
}

pub fn formula(input: &str) {
	statement_result(input, rpn_formula::eval(input));
}

pub fn table_row<T: std::fmt::Display>(contents: &Vec<T>, width: Option<usize>,
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

pub fn table_sep(sep: char, column_count: usize, width: Option<usize>,
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

pub fn table_header<T: std::fmt::Display>(label: &str,
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
		table_sep(middle_sep_out, column_count, Some(width),
			Some('╭'), Some(middle_sep_out), Some('╮'));
		table_row(&vec![label], Some(content_width),
			Some(left), Some(middle), Some(right));
		table_sep('═', column_count, None,
			Some('╞'), Some('╤'), Some('╡'));
		table_row(columns, Some(width),
			Some(left), Some(middle), Some(right));
	}
}


pub fn truth_table(formula: &str) {
	let mut chars: Vec<char> = formula.to_ascii_uppercase().chars().collect();
	let mut variables: Vec<char> =
		chars.extract_if(|c| c.is_ascii_alphabetic()).collect();

	let mut unique_variables: Vec<char> = variables.clone();
	
	let value_count = variables.len();

	let mut values = vec![0; value_count + 1];

	unique_variables.sort();
	unique_variables.dedup();

	let mut rpn = String::from_iter(variables.iter());
	rpn.extend(&chars);

	variables.push('=');
	table_header(formula, &variables, None, None, None, None);
	variables.pop();


	for combination in 0..1 << unique_variables.len() {
		let substituted = rpn_formula::subst_variables(&rpn, &variables,
			&unique_variables, &mut values, combination);

		let mut separator = '─';
		let mut left: Option<char> = None;
		let mut middle: Option<char> = None;
		let mut right: Option<char> = None;
			
		values[value_count] = rpn_formula::eval(&substituted) as i32;

		if combination == 0 {
			separator = '═';
			left = Some('╞'); middle = Some('╪'); right = Some('╡');
		}

		table_sep(separator, value_count + 1, None, left, middle, right);

		table_row(&values, None, None, None, None);
	}

	table_sep('─', value_count + 1,
		None, Some('╰'), Some('┴'), Some('╯'));
}