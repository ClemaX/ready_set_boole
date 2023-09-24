pub fn row<T: std::fmt::Display>(contents: &Vec<T>, width: Option<usize>,
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

pub fn sep(sep: char, column_count: usize, width: Option<usize>,
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

pub fn header<T: std::fmt::Display>(label: &str,
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
		sep(middle_sep_out, column_count, Some(width),
			Some('╭'), Some(middle_sep_out), Some('╮'));
		row(&vec![label], Some(content_width),
			Some(left), Some(middle), Some(right));
		sep('═', column_count, None,
			Some('╞'), Some('╤'), Some('╡'));
		row(columns, Some(width),
			Some(left), Some(middle), Some(right));
	}
}
