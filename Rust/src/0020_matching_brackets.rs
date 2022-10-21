struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for bracket in s.chars() {
            match bracket {
                '(' => stack.push(bracket),
                '[' => stack.push(bracket),
                '{' => stack.push(bracket),
                ')' => match stack.pop() {
                    Some(opening_bracket) if opening_bracket == '(' => continue,
                    _ => return false,
                },
                ']' => match stack.pop() {
                    Some(opening_bracket) if opening_bracket == '[' => continue,
                    _ => return false,
                },
                '}' => match stack.pop() {
                    Some(opening_bracket) if opening_bracket == '{' => continue,
                    _ => return false,
                },
                _ => return false,
            }
        }
        return stack.len() == 0;
    }
}

fn main() {
    let tests = vec!["()", "()[]{}", "(]","({()}()[[]]"];
    for t in tests {
        let result = Solution::is_valid(t.to_string());
        println!("{} -> {}", t, result);
    }
}

