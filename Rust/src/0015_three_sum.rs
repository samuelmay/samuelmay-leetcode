use std::collections::HashSet;
struct Solution;

impl Solution {
	pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
		// Strategy - build on the two sum solution.
		let mut results : Vec<Vec<i32>> = Vec::new();

		// first you gotta sort/partition the thing. Then we need to split it into
		// a POSITIVE slice, ZERO slice, and a NEGATIVE slice.
		let mut positives: Vec<i32> = Vec::new();
		let mut negatives: Vec<i32> = Vec::new();
		let mut unique_positives: HashSet<i32> = HashSet::new();
		let mut unique_negatives: HashSet<i32> = HashSet::new();
		let mut zero_count: i32 = 0;
		
		for n in nums {
			if n > 0 {
				positives.push(n);
				unique_positives.insert(n);
			} else if n < 0 {
				negatives.push(n);
				unique_negatives.insert(n);
			} else {
				zero_count = zero_count + 1;
			}
		}

		// For solutions including a zero, either all three are zeros:
		if zero_count >= 3 {
			results.push(vec![0,0,0]);
		}

		// Or there is a negative number, a zero, and a positive
		// number that has the same absolute value as the negative
		if zero_count >= 1 {
			for n in &unique_negatives {
				let p = -(*n);
				if unique_positives.contains(&p) {
					results.push(vec![*n,0,p]);
				}
			}
		}

		// For non-zero solutions:
		//
		// For every candiate i in the NEGATIVE slice, we know that j and k must be
		// positive and add up to -i . We can do two sum algorithm on the POSITIVE slice.
		//    * d'oh, for each j, search for (i-k)
		for n in &unique_negatives {
			Self::add_if_two_sum_exists(*n,&positives,&mut results);
		}
		
		// For every candiate i in the POSITIVE slice...vice versa.
		for p in &unique_positives {
			Self::add_if_two_sum_exists(*p,&negatives,&mut results);
		}

		return results;
	}
	
	fn add_if_two_sum_exists(target: i32, nums: &[i32], results: &mut Vec<Vec<i32>>) {
		let mut checked : HashSet<i32> = HashSet::new();
		for i in 0..nums.len() {
			if checked.contains(&nums[i]) {
				continue;
			}
			let remainder = -(nums[i]+target);
			for j in (i + 1)..nums.len() {
				if nums[j] == remainder {
					checked.insert(nums[i]);
					checked.insert(nums[j]);
					results.push(vec![target, nums[i], nums[j]]);
					break;
				}
			}
		}
	}
}



fn main() {
	//let test1 = vec![-1,0,1,2,-1,-4];
	let test1 = vec![-1,3,0,1,2,-1,-4,0,0,-5,2,2];
	let result = Solution::three_sum(test1);
	println!("{:?}", result);
}