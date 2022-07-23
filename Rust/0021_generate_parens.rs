struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n < 1 {
            panic!("n must be greater than or equal to 1");
        }
        return Self::generate_parenthesis_recursive(0, n);
    }

    pub fn generate_parenthesis_recursive(open: i32, remaining: i32) -> Vec<String> {
        // if there are no open brackets and no remaining brackets to choose,
        // we can't do anything. base case.
        if open == 0 && remaining == 0 {
            return vec!["".to_string()];
        }

        let mut results = Vec::new();
        // if we have any remaining brackets to open,
        // we can choose to open a new bracket.
        // remaining = remainding-1
        // open = open+1
        if remaining > 0 {
            let choose_open = Self::generate_parenthesis_recursive(open + 1, remaining - 1);
            for result in choose_open {
                results.push(format!("({}", result));
            }
        }
        // if there are any open brackets, we can choose to close
        // open = open - 1
        if open > 0 {
            let choose_close = Self::generate_parenthesis_recursive(open - 1, remaining);
            for result in choose_close {
                results.push(format!("){}", result));
            }
        }
        return results;
    }
}

fn main() {
    let result = Solution::generate_parenthesis(3);
    println!("{:?}", result);
    let r2 = Solution::generate_parenthesis(2);
    println!("{:?}", r2);
}
