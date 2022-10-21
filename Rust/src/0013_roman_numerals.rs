// Not my code - reused as an example to get my toe in the water - see https://medium.com/journey-to-rust/leetcode-excersises-as-a-way-learn-rust-49bee3f37afb
// Copyright George Shuklin
fn roman_to_int(s: String) -> i32 {
	let mut acc = 0;
	let mut stream = s.chars().peekable();
	while let Some(curr) = stream.next(){
		let next = stream.peek();
		acc += match (curr, next){
			('I', Some('V')) => -1,
			('I', Some('X')) => -1,
			('X', Some('L')) => -10,
			('X', Some('C')) => -10,
			('C', Some('D')) => -100,
			('C', Some('M')) => -100,
			('I', _) => 1,
			('V', _) => 5,
			('X', _) => 10,
			('L', _) => 50,
			('C', _) => 100,
			('D', _) => 500,
			('M', _) => 1000,
			_ => panic!("Bad symbol")
		}
	}
	acc
}

fn main() {
	let input1 : String = String::from("XII");
	let result1 : i32 = roman_to_int(input1);
	
	println!("{} -> {}", "XII", result1);

	

}