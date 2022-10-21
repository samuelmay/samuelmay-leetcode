fn is_palindrome(x: i32) -> bool {
	let x_string = x.to_string();
	let mut digits = x_string.chars();
	while let Some((front, back)) = digits.next().zip(digits.next_back()) {
		if front != back {
			return false;
		}
	}
	return true;
}

fn main() {
	let examples : Vec<i32> = vec![121, -121, 10, 6, 21477412];
	for example in examples {
		let result = is_palindrome(example);
		println!("{} -> {}",example,result);
	}
}