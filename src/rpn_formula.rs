use std::collections::HashMap;

use bitvec::prelude::*;

pub struct Node {
	pub token: char,
	pub a: Option<Box<Node>>,
	pub b: Option<Box<Node>>,
}

fn is_operand(token: char) -> bool {
	return token == '0' || token == '1' || token.is_alphabetic();
}

fn is_operator(token: char) -> bool {
	return !is_operand(token);
}

fn is_unary_operator(token: char) -> bool {
	return token == '!';
}

fn is_binary_operator(token: char) -> bool {
	return is_operator(token) && !is_unary_operator(token);
}

pub fn parse_operation(chars: &mut Vec<char>) -> Node {
	let mut root = Node { a: None, b: None, token: '\0' };
	let mut node: Node;

	let token = chars.pop().expect("expected token");
	
	if is_operator(token) {
		root.token = token;
		
		if is_unary_operator(token) {
			node = parse_operation(chars);
			
			if node.token == '\0' {
				panic!("expected operand")					
			}

			root.a = Some(Box::new(node));
		}
		else if is_binary_operator(token) {
			node = parse_operation(chars);

			if node.token == '\0' {
				panic!("expected second operand")					
			}

			root.b = Some(Box::new(node));

			node = parse_operation(chars);

			if node.token == '\0' {
				panic!("expected first operand")					
			}

			root.a = Some(Box::new(node));
		}
	}
	else {
		root.token = token;
	}

	root
}

fn read_operation(node: &Node) -> String {
	let mut operation: String = String::new();

	if is_binary_operator(node.token) {
		let first_operand = read_operation(node.a.as_ref().expect("expected first operand"));
		let second_operand = read_operation(node.b.as_ref().expect("expected second operand"));

		let operator = node.token;

		operation += &format!("({} {} {})", first_operand, operator, second_operand);
	}
	else if is_unary_operator(node.token) {
		let operand = read_operation(node.a.as_ref().expect("expected operand"));

		let operator = node.token;

		operation += &format!("{}{}", operator, operand);
	}
	else {
		operation += &format!("{}", node.token)
	}

	operation
}

pub fn print_tree(root: &Node) {
	let operation = read_operation(root);
	
	println!("{}", operation);
}

pub fn parse(input: &str) -> Node {
	let mut chars: Vec<char> = input.chars().collect();

	let root = parse_operation(&mut chars);

	root
}

pub fn eval(input: &str) -> bool {
	let mut bits = bitvec![];
	//let chars: Vec<char> = input.chars().collect();

	for c in input.chars() {
		match c {
			'0' | '1' => {
				bits.push(c != '0');
			},
			_ => {
				let b = bits.pop().expect("expected first operand!");

				match c {
					'!' => { bits.push(b ^ true); },
					_ => {
						let a = bits.pop().expect("expected second operand!");

						match c {
							'&' => { bits.push(a & b); },
							'|' => { bits.push(a | b); },
							'^' => { bits.push(a ^ b); },
							'>' => { bits.push(a ^ true | b); },
							'=' => { bits.push(a ^ true ^ b); },
							_ => { panic!("unknown operator {:?}", c); },
						};
					},
				};
			}
		};
	}

	bits.pop().expect("expected operation")
}

pub fn subst_variables(formula: &str, variables: &Vec<char>,
	unique_variables: &Vec<char>, values: &mut Vec<i32>, mut combination: i32)
	-> String {
	let mut substituted = formula.to_string();
	let mut variable_values: HashMap<char, i32> = HashMap::new();
	
	for variable_name in unique_variables.iter() {
		let variable_value = combination & 1;

		variable_values.insert(*variable_name, variable_value);

		substituted = substituted.replace(&variable_name.to_string(),
			&variable_value.to_string());
		
		combination >>= 1;
	}

	for (i, variable_name) in variables.iter().enumerate() {
		values[i] = *variable_values.get(variable_name).unwrap();
	}

	return substituted.to_string();
}

/* 
const OP_INVERSE: [(char, char, bool, bool); 4] = [
	('&', '|', true, true),
	('|', '&', true, true),
	('^', '&', true, true), // 1 ^ 1 -> 0 : 1 & 1 -> 1, 1 ^ 0 -> 1 : 0 & 1 -> 1
	('>', '|', true, false),
];

pub fn format_negation_normal(formula: &str) -> String {
	let formatted: String = String::new();

	let mut last_operands: Vec<char>;
	let mut last_op: char = '\0';

	let mut operands: Vec<char> = vec![];
	let mut op: char = '\0';

	for c in formula.chars() {
		if c.is_alphanumeric() {
			operands.push(c);
			continue;
		}

		let b = operands.pop().expect("expected first operand!");

		match c {
			'!' => {  },
			_ => {
				let a = operands.pop().expect("expected second operand!");

				match c {
					'&'| '|' | '^' | '>' | '=' => {
						op = c;
					},
					_ => { panic!("unknown operator {:?}", c); },
				};
			},
		};

		let negate = op == '!';
		
		if last_op != '\0' {
			if (op == '=') {
				
			}
			if negate {
			}
		}

		last_operands = operands;
		last_op = op;

		operands = vec![];
	}
			
	return formatted;
} */