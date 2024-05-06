use core::fmt;
use std::collections::HashMap;

use bitvec::prelude::*;

use crate::ast::Node;

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{:20}", read_operation(self))
    }
}

pub fn is_operand(token: char) -> bool {
	token == '0' || token == '1' || token.is_alphabetic()
}

pub fn is_operator(token: char) -> bool {
	!is_operand(token)
}

pub fn is_unary_operator(token: char) -> bool {
	token == '!'
}

pub fn is_binary_operator(token: char) -> bool {
	is_operator(token) && !is_unary_operator(token)
}

fn parse_operation(chars: &mut Vec<char>) -> Node {
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

fn read_operation_polish(node: &Node) -> String {
	let mut operation: String = String::new();

	if is_binary_operator(node.token) {
		let first_operand = read_operation_polish(
			node.a.as_ref().expect("expected first operand"));
		let second_operand = read_operation_polish(
			node.b.as_ref().expect("expected second operand"));

		let operator = node.token;

		operation += &format!("{}{}{}", operator, second_operand, first_operand);
	}
	else if is_unary_operator(node.token) {
		let operand = read_operation_polish(
			node.a.as_ref().expect("expected operand"));

		let operator = node.token;

		operation += &format!("{}{}", operator, operand);
	}
	else {
		operation += &format!("{}", node.token)
	}

	operation
}

/// Create an RPN string representation of an Abstract Syntax Tree
pub fn read_operation(node: &Node) -> String {
	let operation = read_operation_polish(node);
	
	operation.chars().rev().collect()
}

/* 
fn read_operation_human(node: &Node) -> String {
	let mut operation: String = String::new();

	if is_binary_operator(node.token) {
		let first_operand = read_operation_human(node.a.as_ref().expect("expected first operand"));
		let second_operand = read_operation_human(node.b.as_ref().expect("expected second operand"));

		let operator = node.token;

		operation += &format!("({} {} {})", first_operand, operator, second_operand);
	}
	else if is_unary_operator(node.token) {
		let operand = read_operation_human(node.a.as_ref().expect("expected operand"));

		let operator = node.token;

		operation += &format!("{}{}", operator, operand);
	}
	else {
		operation += &format!("{}", node.token)
	}

	operation
}

pub fn print_human(root: &Node) {
	let operation = read_operation_human(root);
	
	println!("{}", operation);
}
*/

/// Parse an RPN formula as an Abstract Syntax Tree
pub fn parse(input: &str) -> Node {
	let mut chars: Vec<char> = input.chars().collect();

	let root = parse_operation(&mut chars);

	root
}

/// Evaluate an RPN formula
pub fn eval(input: &str) -> bool {
	let mut bits = bitvec![];

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

/// Substitute given variables in a formula from a combination bitfield 
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

	substituted.to_string()
}
