use std::convert::TryFrom;
struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut result_end = 0;
        let mut current_val = 0;
        while current_val < nums.len() {
            while current_val < (nums.len()-1) && nums[current_val] == nums[current_val + 1] {
                current_val = current_val + 1;
            }
            nums[result_end] = nums[current_val];
            result_end = result_end + 1;
            current_val = current_val + 1;
        }
        return i32::try_from(result_end).unwrap();
        //let result_length = result_end;
        // not strictly needed but I'm doing it for neatness
        //while result_end < nums.len() {
        //    nums[result_end] = 0;
        //    result_end = result_end + 1;
        //}
        //return i32::try_from(result_length).unwrap();
    }
}

fn main() {
    let mut test1 = vec![1,2,2,3,3,3,5,6,6];
    let x = Solution::remove_duplicates(&mut test1);
    println!("{} : {:?}",x,test1);
    
    
    let mut test2 = vec![1,1,2];
    let r = Solution::remove_duplicates(&mut test2);
    println!("{} : {:?}",r,test2);
    
    
    let mut test3 = vec![3,3,3,3,5];
    let r2 = Solution::remove_duplicates(&mut test3);
    println!("{} : {:?}",r2,test3);
}