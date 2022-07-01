use std::convert::TryFrom;

// My interpretation of this problem was as an set of integer sequences/patterns.
// I worked out this.
//    Let the number of output rows be 'N'
//    Let the current output row be 'r' (this is a loop variable)
//    Let the current output index be 'i' (also a loop variable)
//    Let the zigzag increment constant be Z = 2*(N-1)
// Then
//    For r = 0 then the input index is
//        i*2*(N-1)            = i*Z
//    For 0 < r < N-1 then the input index has two ouput values
//        i*2*(N-1) + r        = i*Z + r
//        (i + 1) *2*(N-1) - r = (i+1)*Z - r
//    For r = N-1 then the input index is similar but has only the one output value
//        i*2*(N-1) + r        = i*Z + r

fn convert(s: String, num_rows: i32) -> String {
	if num_rows == 1 {
		return s;
	}
	
	let input_length = s.len();
	let input: Vec<char> = s.chars().collect();
	let mut output: Vec<char> = Vec::with_capacity(input_length);
	let N = usize::try_from(num_rows).unwrap();
	let Z = 2*(N - 1);
	
	let mut i = 0usize;
	// First row - case r = 0
	loop {
		let input_position = i*Z;
		if input_position >= input_length {
			break;
		}
		output.push(input[input_position]);
		i += 1;
	}
	// Intermediate rows. Case 0 < r < N - 2
	for r in 1..(N-1) {
		i = 0;
		loop {
			let input_position_1 = i*Z + r;
			if input_position_1 >= input_length {
				break;
			}
			output.push(input[input_position_1]);

			let input_position_2 = (i+1)*Z - r;
			if input_position_2 < input_length {
				output.push(input[input_position_2]);
			}
			
			i += 1;
		}
	}

	// Final row, Case r = N - 1
	i = 0;
	let r = N - 1;
	loop {
		let input_position = i*Z + r;
		if input_position >= input_length {
			break;
		}
		output.push(input[input_position]);
		i += 1;
	}
	
	return output.into_iter().collect();
	
}

fn main () {
	let examples = vec![("PAYPALISHIRING",3),("PAYPALISHIRING",4),("A",1),("AB",2)];
	for (example,num_rows) in examples {
		let s = String::from(example);
		let result = convert(s,num_rows);
		println!("{}",result);
	}
}