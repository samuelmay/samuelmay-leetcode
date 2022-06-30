fn is_palindrome(string_slice: &str) -> bool {
	let mut iterator = string_slice.chars();
	while let Some((front, back)) = iterator.next().zip(iterator.next_back()) {
		if front != back {
			return false;
		}
	}
	return true;
}

fn longest_palindrome_recursive(string_slice: &str) -> &str {
	if string_slice.len() <= 1 || is_palindrome(string_slice) {
		return string_slice;
	} else {
		let mut substring_end = string_slice.len() - 1; // slice to second last element
		let mut left_palindrome : &str = &string_slice[0..1]; // initialize with just the first character as a base case
		while substring_end > 1 {
			let candidate_palindrome = &string_slice[0..substring_end];
			if is_palindrome(candidate_palindrome) {
				left_palindrome = candidate_palindrome;
				break;
			}
			substring_end -= 1;
		}
		
		let right_palindrome : &str = longest_palindrome_recursive(&string_slice[1..]);

		if left_palindrome.len() < right_palindrome.len() {
			return right_palindrome;
		} else {
			return left_palindrome;
		}
	}
}

fn longest_palindrome(s: String) -> String {
	let s_as_slice = &s[..];
	let result = longest_palindrome_recursive(s_as_slice);
	return result.to_string();
}

fn main () {
	let examples = vec!["babad","cbbd","bb"];
	for example in examples {
		let result = longest_palindrome_recursive(example);
		println!("{}",result);
	}
}
