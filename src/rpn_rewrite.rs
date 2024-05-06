use lazy_static::lazy_static;
use std::{collections::HashMap, ops::Deref};

use crate::{ast::Node, rpn_formula::{is_operator, parse, read_operation}};

pub struct RewriteRule {
	pub pattern: Node,
	pub substitute: Node,
}

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

fn tree_find<'a>(node: &'a mut Node, pattern: &Node,
  propositional: &mut HashMap<char, Node>, recurse: bool)
  -> Option<&'a mut Node> {
  let mut found_node: Option<&mut Node> = None;
  

  if is_operator(pattern.token) {
    let mut matches = node.token == pattern.token;

    if matches {
      // Check if children match too
      if let Some(pattern_a) = &pattern.a {
        matches = node.a.as_mut().is_some_and(|node_a|
          tree_find(node_a, &pattern_a, propositional, false).is_some());

        if matches {
          if let Some(pattern_b) = &pattern.b {
            matches = node.b.as_mut().is_some_and(|node_b|
              tree_find(node_b, &pattern_b, propositional, false).is_some());
          }
          else {
            matches = node.b.is_none()
          }
        }
      }

      if matches {
        return Some(node);
      }
    }
  }
  else if pattern.token.is_alphabetic() {
    propositional.insert(pattern.token, node.clone());
    return Some(node);
  }
  else {
    panic!("unknown token '{}'", pattern.token);
  }

  if found_node.is_none() && recurse {
    // Search in children
    if let Some(node_a) = node.a.as_mut() {
      found_node = tree_find(node_a, pattern, propositional, recurse)
    }

    if found_node.is_none() && let Some(node_b) = node.b.as_mut() {
      found_node = tree_find(node_b, pattern, propositional, recurse)
    }
  }

  found_node
}

/// Rewrite operands according to propositional subtree mapping recursively
fn rewrite_operands(node: &mut Node, propositional: &mut HashMap<char, Node>) {
	if let Some(subtree) = propositional.get(&node.token) {
    node.token = subtree.token;
    node.a = subtree.a.clone();
    node.b = subtree.b.clone();
  }
  else {
    if let Some(a) = node.a.as_mut() {
      rewrite_operands(a, propositional);
    }
    if let Some(b) = node.b.as_mut() {
      rewrite_operands(b, propositional);
    }
  }
}

/// Find patterns in tree and rewrite operations until no rule matches
pub fn tree_rewrite<'a>(node: &'a mut Node, rules: &[RewriteRule]) {
  let mut propositional: HashMap<char, Node> = HashMap::with_capacity(2);
  let mut completed: bool = false;

	while !completed {
		completed = true;

		for rule in rules {
      propositional.clear();

      // Find pattern and get operand subtrees
      if let Some(matching_node) = tree_find(node, &rule.pattern, &mut propositional, true) {
        matching_node.token = rule.substitute.token;
        matching_node.a = rule.substitute.a.clone();
        matching_node.b = rule.substitute.b.clone();

        rewrite_operands(node, &mut propositional);

        completed = false;
      }
		}
	}
}

pub fn rewrite(formula: &str, rules: &[RewriteRule]) -> String {
	let mut root = parse(formula);

	tree_rewrite(&mut root, rules);

	read_operation(&root)
}

pub fn parse_rewrite(pattern: &str, substitute: &str) -> RewriteRule {
  RewriteRule { // AB> -> A!B|
    pattern: parse(pattern),
    substitute: parse(substitute)
  }
}

pub fn negation_normal_form(formula: &str) -> String {
	rewrite(formula, REWRITE_RULES_NNF.deref())
}
