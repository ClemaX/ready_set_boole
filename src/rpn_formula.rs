use std::{collections::HashMap, ops::Deref};

use bitvec::prelude::*;
use lazy_static::lazy_static;

const NODE_SYM_A: char = '\x01';
const NODE_SYM_B: char = '\x02';

lazy_static! {
  static ref REWRITE_RULES_NNF: [RewriteRule; 6] = [
    parse_rewrite("AB>", "A!B|"),
    parse_rewrite("AB=", "A!B|AB!|&"),
    parse_rewrite("AB^", "A!B&AB!&|"),
    parse_rewrite("AB|!", "A!B!&"),
    parse_rewrite("AB&!", "A!B!|"),
    parse_rewrite("A!!", "A"),
	];
}

#[derive(Debug, Clone)]
pub struct Node {
	pub token: char,
	pub a: Option<Box<Node>>,
	pub b: Option<Box<Node>>,
}

fn is_operand(token: char) -> bool {
	token == '0' || token == '1' || token.is_alphabetic()
}

fn is_operator(token: char) -> bool {
	!is_operand(token)
}

fn is_unary_operator(token: char) -> bool {
	token == '!'
}

fn is_binary_operator(token: char) -> bool {
	is_operator(token) && !is_unary_operator(token)
}

pub fn parse_operation(chars: &mut Vec<char>, tr_operand: Option<fn(char) -> char>) -> Node {
	let mut root = Node { a: None, b: None, token: '\0' };
	let mut node: Node;

	let token = chars.pop().expect("expected token");
	
	if is_operator(token) {
		root.token = token;
		
		if is_unary_operator(token) {
			node = parse_operation(chars, tr_operand);
			
			if node.token == '\0' {
				panic!("expected operand")					
			}

			root.a = Some(Box::new(node));
		}
		else if is_binary_operator(token) {
			node = parse_operation(chars, tr_operand);

			if node.token == '\0' {
				panic!("expected second operand")					
			}

			root.b = Some(Box::new(node));

			node = parse_operation(chars, tr_operand);

			if node.token == '\0' {
				panic!("expected first operand")					
			}

			root.a = Some(Box::new(node));
		}
	}
	else {
		root.token = match tr_operand {
      None => token,
      Some(tr) => tr(token)
    };
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

pub fn parse_tr(input: &str, transform_operand: fn(char) -> char) -> Node {
	let mut chars: Vec<char> = input.chars().collect();

	let root = parse_operation(&mut chars, Some(transform_operand));

	root
}

pub fn parse(input: &str) -> Node {
	let mut chars: Vec<char> = input.chars().collect();

	let root = parse_operation(&mut chars, None);

	root
}

pub struct RewriteRule {
	pub pattern: Node,
	pub substitute: Node,
}

fn rewrite_operands(node: &mut Node, a: &Option<Box<Node>>, b: &Option<Box<Node>>) {
	match node.token {
		NODE_SYM_A => {
			node.token = a.as_ref().unwrap().token;
			node.a = a.as_ref().unwrap().a.clone();
			node.b = a.as_ref().unwrap().b.clone();
		},
		NODE_SYM_B => {
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
				NODE_SYM_A => {
					*a = Some(child.unwrap());
				}
				NODE_SYM_B => {
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

pub fn symbolize_node_name(char: char) -> char {
  match char {
    'A' => '\x01',
    'B' => '\x02',
    _ => panic!("Invalid node name!")
  }
}

pub fn parse_rewrite(pattern: &str, substitute: &str) -> RewriteRule {
  RewriteRule { // AB> -> A!B|
    pattern: parse_tr(pattern, symbolize_node_name),
    substitute: parse_tr(substitute, symbolize_node_name)
  }
}

pub fn negation_normal_form(formula: &str) -> String {
	rewrite(formula, REWRITE_RULES_NNF.deref())
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

	substituted.to_string()
}
