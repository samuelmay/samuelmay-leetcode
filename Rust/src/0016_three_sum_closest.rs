struct Solution;

impl Solution {
	pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
		let mut sorted_nums = nums;
		sorted_nums.sort();

		let mut closest = sorted_nums[0] + sorted_nums[1] + sorted_nums[2];
		let mut min_diff = (target - closest).abs();
		
		for i in 0..(sorted_nums.len()-2) {
			// skip duplicates
			if i > 0 && sorted_nums[i-1] == sorted_nums[i] {
				continue;
			}

			let mut left_pointer = i + 1;
			let mut right_pointer = sorted_nums.len() - 1;

			while left_pointer < right_pointer {
				let this_guess = sorted_nums[i] + sorted_nums[left_pointer] + sorted_nums[right_pointer];
				let diff = (target - this_guess).abs();

				if diff < min_diff {
					closest = this_guess;
					min_diff = diff;
				}
				
				if this_guess > target {
					right_pointer = right_pointer - 1;
				} else if this_guess < target {
					left_pointer = left_pointer + 1;
				} else {
					// hole in one haha
					return this_guess;
				}
			}
		}
		return closest;
	}
}

fn main() {
    let nums = vec![-1,2,1,-4];
	let target = 1;
	let result = Solution::three_sum_closest(nums.clone(),target);
	println!("target is {}",target);
	println!("{:?} -> {}",nums,result);
}