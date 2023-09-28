use std::collections::HashMap;

use bitvec::prelude::*;

#[derive(Debug, Clone)]
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
 */

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

pub fn read_operation(node: &Node) -> String {
	let operation = read_operation_polish(node);
	
	operation.chars().rev().collect()
}

/* 
pub fn print_human(root: &Node) {
	let operation = read_operation_human(root);
	
	println!("{}", operation);
}


pub fn print(root: &Node) {
	let operation = read_operation(root);
	
	println!("{}", operation);
} */

pub fn parse(input: &str) -> Node {
	let mut chars: Vec<char> = input.chars().collect();

	let root = parse_operation(&mut chars);

	root
}

pub struct RewriteRule {
	pub pattern: Node,
	pub substitute: Node,
}

fn rewrite_operands(node: &mut Node, a: &Option<Box<Node>>, b: &Option<Box<Node>>) {
	match node.token {
		'\x01' => {
			node.token = a.as_ref().unwrap().token;
			node.a = a.as_ref().unwrap().a.clone();
			node.b = a.as_ref().unwrap().b.clone();
		},
		'\x02' => {
			node.token = b.as_ref().unwrap().token;
			node.a = b.as_ref().unwrap().a.clone();
			node.b = b.as_ref().unwrap().b.clone();
		},
		_ => {
			if node.a.is_some() {
				rewrite_operands(node.a.as_mut().unwrap(), a, b);
			}
			if node.b.is_some() {
				rewrite_operands(node.b.as_mut().unwrap(), a, b);
			}
		},
	}
}

fn find_pattern_operands(pattern: &Node, node: &mut Node, a: &mut Option<Box<Node>>, b: &mut Option<Box<Node>>) -> bool {
	let mut found = true;

	let pairs = [
		(pattern.a.as_ref(), node.a.clone()),
		(pattern.b.as_ref(), node.b.clone()),
	];

	let mut pattern_token: char;

	for (pattern_child, child) in pairs {
		if pattern_child.is_some() {
			if child.is_none() {
				continue;
			}

			pattern_token = pattern_child.unwrap().token;

			match pattern_token {
				'\x01' => {
					*a = Some(child.unwrap());
				}
				'\x02' => {
					*b = Some(child.unwrap());
				}
				_ => {
					if !is_operator(pattern_token) {
						panic!("unknown token '{}'", pattern_token);
					}

					found = pattern_token == child.as_ref().unwrap().token;
					
					if found {
						if pattern.a.is_some() {
							found = find_pattern(pattern.a.as_ref().unwrap(),
								node.a.as_mut().unwrap(), a, b, false).is_some();
						}
						
						if found && pattern.b.is_some() {
							found = find_pattern(pattern.b.as_ref().unwrap(),
								node.b.as_mut().unwrap(), a, b, false).is_some();
						}
					}
				}
			}

			if !found {
				break;
			}
		}
	}

	found
}

fn find_pattern<'a, >(pattern: &'a Node, node: &'a mut Node,
	a: & mut Option<Box<Node>>, b: & mut Option<Box<Node>>, recurse: bool)
	-> Option<&'a mut Node> {
	let mut found_node: Option<&mut Node> = None;

	if node.token != pattern.token {
		if !recurse {
			return None;
		}

		if recurse {
			match node.a.as_mut() {
				Some(node_a) => {
					found_node = find_pattern(pattern, node_a, a, b, recurse);
			
					match &found_node {
						Some(_) => {},
						None => {
							match node.b.as_mut() {
								Some(found_b) => {
									found_node = find_pattern(pattern, found_b, a, b, recurse);
								},
								None => {},
							}
						},
					}
				},
				None => {},
			}
		}

		return found_node;
	}

	if !find_pattern_operands(pattern, node, a, b) {
		return None;
	}

	Some(node)
}

pub fn rewrite_tree(node: &mut Node, rules: &[RewriteRule]) {
	let mut a: Option<Box<Node>> = None;
	let mut b: Option<Box<Node>> = None;
	let mut completed: bool = false;

	if node.a.is_some() {
		rewrite_tree(node.a.as_mut().unwrap(), rules);
	}

	if node.b.is_some() {
		rewrite_tree(node.b.as_mut().unwrap(), rules);
	}

	while !completed {
		completed = true;

		for rule in rules {
			match find_pattern(&rule.pattern, node, &mut a, &mut b, true) {
				Some(matching) => {
					matching.token = rule.substitute.token;
					matching.a = rule.substitute.a.clone();
					matching.b = rule.substitute.b.clone();
					
					rewrite_operands(node, &a, &b);
	
					completed = false;
				},
				None => {},
			}

		}
	}
}

pub fn rewrite(formula: &str, rules: &[RewriteRule]) -> String {
	let mut root = parse(formula);

	rewrite_tree(&mut root, rules);

	read_operation(&root)
}

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