use std::collections::HashMap;

use bitvec::prelude::*;

pub fn eval(input: &str) -> bool {
	let mut bits = bitvec![];
	//let chars: Vec<char> = input.chars().collect();

	for c in input.chars() {
		match c {
			'0' | '1' => {
				bits.push(c != '0');
			},
			_ => {
				let b = bits.pop().expect("missing first operand!");

				match c {
					'!' => { bits.push(b ^ true); },
					_ => {
						let a = bits.pop().expect("missing second operand!");

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

	bits.pop().expect("missing operation")
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