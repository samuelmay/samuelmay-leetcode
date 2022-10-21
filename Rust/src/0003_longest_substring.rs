#![allow(non_snake_case)]

use std::collections::HashSet;
use std::cmp::max;

fn length_of_longest_substring(s: String) -> i32 {
	let allChars : Vec<char> = s.chars().collect();
	let mut currentSubString: HashSet<char>;
	let mut longestLength: usize = 0;
	for (i,_startChar) in allChars.iter().enumerate() {
		currentSubString = HashSet::new();
		for thisChar in &allChars[i..] {
			if currentSubString.contains(&thisChar)  {
				break;
			}
			currentSubString.insert(*thisChar);
		}
		longestLength = max(longestLength, currentSubString.len());
	}
	return longestLength as i32;
}

fn main() {
	let examples = vec!["abcabcbb", "bbbbb", "pwwkew", "hello baby", "$$%^^", " ", "dvdf"];
	
	for example in examples {
		let result = length_of_longest_substring(example.to_string());
		println!("{} -> {}", example, result);
	}
}