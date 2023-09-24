use bitvec::prelude::*;

pub fn eval(input: &str) -> bool {
	let mut bits = bitvec![];
	let mut chars: Vec<char> = input.chars().collect();
	let operands = chars.extract_if(|c| *c == '0' || *c == '1');

	bits.extend(operands.map(|c| c != '0'));

	for op in chars {
		let b = bits.pop().expect("missing first operand!");

		match op {
			'!' => { bits.push(b ^ true); },
			_ => {
				let a = bits.pop().expect("missing second operand!");

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

pub fn subst_variables(formula: &str, variables: &Vec<char>,
	values: &mut Vec<i32>, mut combination: i32) -> String {
	let mut substituted = formula.to_string();
	
	for (i, variable) in variables.iter().enumerate() {
		let variable_value = combination & 1;

		values[i] = variable_value;

		substituted = substituted.replace(&variable.to_string(),
			&variable_value.to_string());
		combination >>= 1;
	}

	return substituted.to_string();
}