
fn merge_sorted(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
	let total_length = nums1.len() + nums2.len();
	let mut merged : Vec<i32> = Vec::with_capacity(nums1.len() + nums2.len());
	let mut i = 0;
	let mut j = 0;
	for _ in 0..total_length {
		if i >= nums1.len() {
			merged.push(nums2[j]);
			j += 1;
		} else if j >= nums2.len() {
			merged.push(nums1[i]);
			i += 1;
		} else if nums1[i] > nums2[j] {
			merged.push(nums2[j]);
			j += 1;
		} else {
			merged.push(nums1[i]);
			i += 1;
		}
	}
	return merged;
}

fn median(nums: Vec<i32>) -> f64 {
	let length = nums.len();
	if length == 1 {
		return nums[0] as f64;
	} else if length % 2 == 0 {
		// even length
		let sum = nums[length/2] + nums[(length/2) - 1];
		return (sum as f64) / 2f64;
	} else {
		// odd length
		return nums[length/2] as f64;
	}
}
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
	let merged: Vec<i32> = merge_sorted(nums1, nums2);
	let result = median(merged);
	return result;
}

fn main() {
	let t1 = vec![1,3,5];
	let t2 = vec![7];
	let t3 = merge_sorted(t1,t2);
	println!("{:?}", t3);
	
	let mvec1 = vec![1,3,5];
	let m1 = median(mvec1);
	let mvec2 = vec![2,4];
	let m2 = median(mvec2);
	println!("1,3,5 median -> {}",m1);
	println!("2,4 median -> {}",m2);
	
	let e1 = vec![1,2];
	let e2 = vec![3,4];
	let result = find_median_sorted_arrays(e1,e2);
	println!("test result = {}",result);
}