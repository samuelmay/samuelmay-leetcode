struct Solution;

impl Solution {
	pub fn letter_combinations(digits: String) -> Vec<String> {
		let mut chars = digits.chars();
		match chars.next() {
			None => Vec::new(),
			Some(c) => {
				let possible_chars : &str = match c {
					'0' => " ",
					'1' => "",
					'2' => "abc",
					'3' => "def",
					'4' => "ghi",
					'5' => "jkl",
					'6' => "mno",
					'7' => "pqrs",
					'8' => "tuv",
					'9' => "wxyz",
					_ => panic!()
				};

				let mut rest_of_combinations = Self::letter_combinations(chars.as_str().to_string());
				if possible_chars.len() == 0 {
					return rest_of_combinations;
				}
				if rest_of_combinations.len() == 0 {
					rest_of_combinations.push(String::new());
				}

				
				let mut result = Vec::with_capacity(possible_chars.len()*rest_of_combinations.len());
				for p in possible_chars.chars() {
					for s in &rest_of_combinations {
						let new_s = format!("{}{}",p,s);
						result.push(new_s);
					}
				}
				result
			}
		}
	}
}

fn main() {
	let test = String::from("23");
	let result = Solution::letter_combinations(test);
	println!("{:?}", result);
}