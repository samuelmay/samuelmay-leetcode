use std::str::Chars;

struct Solution;

impl Solution {
	pub fn longest_common_prefix(strs: Vec<String>) -> String {
		let mut str_iters : Vec<Chars> = strs.iter().map(|s| s.chars()).collect();
		let mut prefix = String::new();
		loop {
			let mut next_char : Option<char> = None;
			for str_iter in &mut str_iters {
				match (next_char,str_iter.next()) {
					(_,None) => return prefix,
					(None,Some(c)) => next_char = Some(c),
					(Some(n),Some(c)) if n != c => return prefix,
					_ => continue 
				}
			}
			match next_char {
				None => return prefix,
				Some(n) => prefix.push(n)
			}
		}
	}
}

fn main() {
	let examples : Vec<&str> = vec!["dog","racecar","car"];
	let input = examples.iter().map(|a| { a.to_string() }).collect();
	
	let result = Solution::longest_common_prefix(input);
	println!("{}", result);
}