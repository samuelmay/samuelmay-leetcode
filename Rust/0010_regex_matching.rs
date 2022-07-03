type StateIndex = usize;
type TransitionIndex = usize;

struct FiniteAutomata {
	initial_state: StateIndex,
	final_state: StateIndex,
	states: Vec<AutomataState>,
	transitions: Vec<AutomataTransition>,
}

struct AutomataState {
	first_transition: Option<TransitionIndex>,
}

struct AutomataTransition {
	match_symbol: char,
	target: StateIndex,
	next_outgoing_transition: Option<TransitionIndex>
}

struct NextStates<'a> {
	automata : &'a FiniteAutomata,
	symbol: char,
	current_transition_index: Option<TransitionIndex>
}

impl<'a> Iterator for NextStates<'a> {
	type Item = TransitionIndex;

	fn next(&mut self) -> Option<TransitionIndex> {
		loop {
			if self.current_transition_index == None {
				return None;
			}
			let transition_num = self.current_transition_index.unwrap();
			let transition = &self.automata.transitions[transition_num];
			self.current_transition_index = transition.next_outgoing_transition;
			match (self.symbol, transition.match_symbol) {
				('*','*') => return Some(transition.target),
				('*',_) => continue,
				(_,'.') => return Some(transition.target), // dot matches any symbol
                (_,_) if transition.match_symbol == self.symbol => return Some(transition.target),
				(_,_) => continue
			}
		}
	}
}

impl FiniteAutomata {
	fn add_state(&mut self) -> StateIndex {
		let index = self.states.len();
		self.states.push(AutomataState { first_transition: None });
		return index;
	}
	
	fn add_transition(&mut self, source: StateIndex, target: StateIndex, symbol: char) {
		let transition_num = self.transitions.len();
		let state_data = &mut self.states[source];
		self.transitions.push(AutomataTransition {
			match_symbol: symbol,
			target: target,
			next_outgoing_transition: state_data.first_transition
		});
		state_data.first_transition = Some(transition_num);
	}
	
	fn transition_states(&self, source: StateIndex, match_symbol: char) -> NextStates {
		let first_transition = self.states[source].first_transition;
		return NextStates {
			symbol: match_symbol,
			automata: self,
			current_transition_index: first_transition
		}
	}

}

fn compile(pattern: String) -> FiniteAutomata {
	let mut automata = FiniteAutomata {
		initial_state: 0,
		final_state: 0,
		states: Vec::new(),
		transitions: Vec::new()
	};
	let mut current_state : StateIndex;
	let mut stream = pattern.chars();
	
	// start state always exists
	automata.initial_state = automata.add_state();
	current_state = automata.initial_state;
	println!("Adding initial state {}", automata.initial_state);
	
	while let Some(symbol) = stream.next() {
		match symbol {
			'*' => {
				// Kleene star
				// Add 'Kleene closure' to previous state
				
				
			}
			_ => {
				let next_state = automata.add_state();
				println!("Adding state {}",next_state);
				automata.add_transition(current_state, next_state, symbol);
				println!("Adding transition {} -> {} -> {}",current_state, symbol, next_state);
				current_state = next_state;
			}
		}
	}
	automata.final_state = current_state;
	println!("Final state is {}", automata.final_state);
	return automata;
}

fn is_match(s: String, p: String) -> bool {
	let automata : FiniteAutomata = compile(p);

	let mut current_state : StateIndex;
	current_state = automata.initial_state;
	println!("Current state is {}", current_state);
	
	for i in s.chars() {
		for next_state in automata.transition_states(current_state, i) {
			current_state = next_state;
		}
		println!("Current state is {}", current_state);
	}
	println!("Testing against final state {}...", automata.final_state);
	return current_state == automata.final_state;
}

fn main() {
	let examples = vec!["aaa","","abc","axc"];
	let pattern = "a.c";

	for example in examples {
		let p = String::from(pattern);
		let result = is_match(example.to_string(),p);
		println!("{} -> {}", example, result);
	}
}