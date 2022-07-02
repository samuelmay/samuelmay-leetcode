fn compile(pattern: String) -> (usize, Box<dyn Fn(usize,char) -> usize>) {
	let mut state_transitions : Vec<Box<dyn Fn(char) -> usize>> = Vec::new();
	let mut current_state : usize = 0;
	let mut stream = pattern.chars().peekable();
	
	while let Some(c) = stream.next() {
		let peek_star = stream.peek();
		let state = current_state;
		if let Some(star) = peek_star {
			if *star == '*' {
				// compile a multiple match function
				// Consume the star metacharacter '*' and peek ahead again.
				let _ = stream.next();
				let peek_next_c = stream.peek().cloned();
				if let Some(next_c) = peek_next_c {
					// we have a future character that causes us to transition out of the greedy
					// matching state.

					// consume the character that ends the greedyness
					let _ = stream.next();
					let test_char = next_c;

					if c == '.' {
						// wildcard matches anything
						state_transitions.push(Box::new(move |i : char| {
							if i == test_char {
								return state + 1;
							} else {
								return state;
							}
						}));
					} else {
						// the next character needs to be matched
						// specifically
						state_transitions.push(Box::new(move |i : char| {
							if i == c {
								return state;
							} else if i == test_char {
								return state + 1;
							} else {
								return 0;
							}
						}));
					}
				} else {
					// there is a greedy star at the end of the regexp
					// This is a trivial case that always matches
					state_transitions.push(Box::new(move |_i : char| state + 1));
				}
				current_state += 1;
				continue;
			}
		}

		// No greedy star modifier
		if c == '.' {
			// wildcard! match anything
			state_transitions.push(Box::new(move |_i : char| state + 1));
		} else {
			state_transitions.push(Box::new(move |i : char| {
				if i == c {
					return state + 1;
				} else {
					return 0
				}
			}));
		}
		current_state += 1;
	}
	
		
	let final_state = current_state;
	let delta_function = Box::new(move |q : usize, i : char| {
		return state_transitions[q](i);
	});

	return (final_state,delta_function);
}
fn is_match(s: String, p: String) -> bool {
	// we want to match the exact string length but I had already
	// gone the whole hog for partial matching so this is a quick hack
	// to fix it. We are already adding a '\0' null terminator to every
	// string given.
	let mut s_terminated = s.clone();
	s_terminated.push('\0'); 
	let mut p_terminated = p.clone();
	p_terminated.push('\0');
	
	let (end_state,delta) = compile(p_terminated);
	let mut q = 0;
	for i in s_terminated.chars() {
		q = delta(q,i);
		if q == end_state {
			return true;
		}
	}
	return false;
}

fn main() {
	let examples = vec!["baab","bbc","","bb"];
	let pattern = "ba*b";

	for example in examples {
		let p = String::from(pattern);
		let result = is_match(example.to_string(),p);
		println!("{} -> {}", example, result);
	}
}