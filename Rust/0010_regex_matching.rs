fn compile(pattern: String) -> (usize, Box<dyn Fn(usize,char) -> usize>) {
	let mut state_transitions : Vec<Box<dyn Fn(char) -> usize>> = Vec::new();
	let mut current_state : usize = 0;
	let mut stream = pattern.chars();
	
	while let Some(c) = stream.next() {
		let state = current_state;
		let transition_function = move |i : char| {
			if i == c {
				return state + 1;
			} else {
				return 0
			}
		};
		state_transitions.push(Box::new(transition_function));
		current_state += 1;
	}
	
	let final_state = current_state;
	let delta_function = Box::new(move |q : usize, i : char| {
		return state_transitions[q](i);
	});

	return (final_state,delta_function);
}
fn is_match(s: String, p: String) -> bool {
	let (end_state,delta) = compile(p);
	let mut q = 0;
	for i in s.chars() {
		q = delta(q,i);
		if q == end_state {
			return true;
		}
	}
	return false;
}

fn main() {
	let examples = vec!["quick brown faox","triple aaa gaming","aa battery"];
	let pattern = "brown";

	for example in examples {
		let p = String::from(pattern);
		let result = is_match(example.to_string(),p);
		println!("{} -> {}", example, result);
	}
}