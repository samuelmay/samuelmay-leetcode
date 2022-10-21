fn int_to_roman(num: i32) -> String {
	let mut num_remainder = num;
	let mut roman_numeral : Vec<char> = Vec::new();

	let symbols = ['M','D','C','L','X','V','I'];
	let values = [1000,500,100,50,10,5,1];

	for i in 0..symbols.len() {
		let symbol_repeats = num_remainder / values[i];
		let remainder = num_remainder % values[i];

		if i > 0 && symbol_repeats == 4 {
			let subtraction_symbol = match (roman_numeral.last(),symbols[i]) {
				(Some('V'),'I') => { roman_numeral.pop(); 'X' },
				(Some('L'),'X') => { roman_numeral.pop(); 'C' },
				(Some('D'),'C') => { roman_numeral.pop(); 'M' },
				(_,'I') => 'V',
				(_,'X') => 'L',
				(_,'C') => 'D',
				_ => panic!("Impossible subtraction combination")
			};
			roman_numeral.push(symbols[i]);
			roman_numeral.push(subtraction_symbol);
		} else {
			for _ in 0..symbol_repeats {
				roman_numeral.push(symbols[i]);
			}
		}
		num_remainder = remainder;
	}
	return roman_numeral.into_iter().collect();
}

fn main() {
	let examples = [ 1, 9, 12, 17, 53, 88, 3050, 3, 58, 4090 ];
	for e in examples {
		let r = int_to_roman(e);
		println!("{}",r);
	}
}