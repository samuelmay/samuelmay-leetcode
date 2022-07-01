use std::convert::TryFrom;

fn reverse (x: i32) -> i32 {
	let bigger_x = x as i64;
	let (reversed_x,_) = reverse_recursive(bigger_x);
	return match i32::try_from(reversed_x) {
		Ok(result) => result,
		Err(_e) => 0i32
	}
}

fn reverse_recursive(x: i64) -> (i64,i64) {
	let least_signifiant_digit = x % 10;
	let other_digits = x / 10;
	if other_digits == 0 {
		return (least_signifiant_digit,10);
	} else {
		let (reversed_other_digits,exponent) = reverse_recursive(other_digits);

		let all_reversed = least_signifiant_digit * exponent + reversed_other_digits;
		let new_exponent = exponent * 10;

		return (all_reversed, new_exponent);
	}
}

fn main() {
	let examples : Vec<i32> = vec![439, 1000, 98705, -616, 2147400007];
	for example in examples {
		let result = reverse(example);
		println!("{} -> {}",example,result);
	}
}