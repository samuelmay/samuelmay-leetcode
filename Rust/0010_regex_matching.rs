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
		//println!("Adding state {}",index);
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
		//println!("Adding transition {} -> {} -> {}",source, symbol, target);
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
	let mut stream = pattern.chars().peekable();
	
	// start state always exists
	automata.initial_state = automata.add_state();
	current_state = automata.initial_state;
	
	while let Some(symbol) = stream.next() {
		let next_symbol = stream.peek();
		match (symbol,next_symbol) {
			(_,Some('*')) => {
				// Kleene star
				// Add 'Kleene closure'
				let state_inner_1 = automata.add_state();
				let state_inner_2 = automata.add_state();
				let skip_state = automata.add_state();

				// add the skip link
				automata.add_transition(current_state, skip_state, '*');
				// add the link to the inner, symbol-matching transition 
				automata.add_transition(current_state, state_inner_1, '*');
				// add the symbol-matching transition
				automata.add_transition(state_inner_1, state_inner_2, symbol);
				// add the loopback link
				automata.add_transition(state_inner_2, state_inner_1, '*');
				// add the exit link
				automata.add_transition(state_inner_2, skip_state, '*');
				current_state = skip_state;
			},
			('*',_) => continue, // skip the Kleene star as it was already processed when we peeked at it
			(_,_) => {
				let next_state = automata.add_state();
				automata.add_transition(current_state, next_state, symbol);
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

	let mut current_states : Vec<StateIndex> = Vec::new();
	let mut visited : Vec<StateIndex> = Vec::new();
	search_next_states(&automata, automata.initial_state, &mut current_states, &mut visited);
	println!("Current states is {:?}", current_states);
	
	for i in s.chars() {
		let previous_states = current_states.clone();
		current_states.clear();
		for state in previous_states {
			for next_state in automata.transition_states(state, i) {
				visited.clear();
				search_next_states(&automata, next_state, &mut current_states, &mut visited);
			}
		}
		println!("Current states is {:?}", current_states);
		if current_states.len() == 0 {
			return false;
		}
	}
	println!("Testing against final state {}...", automata.final_state);
	return current_states.contains(&automata.final_state);
}

fn search_next_states(
		automata : &FiniteAutomata,
		start : StateIndex,
		next_states : &mut Vec<StateIndex>,
		visited : &mut Vec<StateIndex>
	) {
	let epsilon_transitions : Vec<StateIndex> = automata.transition_states(start,'*').collect();
	if epsilon_transitions.len() == 0 {
		next_states.push(start);
	} else {
		for state in epsilon_transitions {
			if !visited.contains(&state) {
				visited.push(state);
				search_next_states(automata,state,next_states,visited);
			}
		}
	}
}

fn main() {
	let examples = vec!["ab"];
	let pattern = ".*";

	for example in examples {
		let p = String::from(pattern);
		let result = is_match(example.to_string(),p);
		println!("    = {} -> {}", example, result);
	}
}