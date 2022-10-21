
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() < needle.len()  || haystack.len() == 0 {
            return -1; // needle is longer than haystack
        }
        // if haystack and needle are the same length, we still need
        // to take at least one character from haystack
        let search_finish: usize = haystack.len() - needle.len() + 1;
        match needle.chars().next() {
            None => return 0, // trivial match, needle is empty string
            Some(match_start) => {
                for (i, c) in haystack.chars().take(search_finish).enumerate() {
                    if c == match_start && haystack[i..i+needle.len()].eq(&needle[..]) {
                        return i as i32;
                    }
                }
            }
        }
        return -1;
    }
}

struct Solution;

fn main() {
	let examples : Vec<(&str,&str)> = vec![
        ("sadbutsad","sad"),
        ("leetcode","leeto"),
        ("leetcode","tc"),
        ("mississippi","issip"),
        ("mississippi","sippia")
    ];
	for example in examples {
		let result = Solution::str_str(String::from(example.0),String::from(example.1));
		println!("{} in {} -> {}",example.1, example.0, result);
	}
}
