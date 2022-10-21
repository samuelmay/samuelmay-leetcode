fn max_area(height: Vec<i32>) -> i32 {
	let mut max : i32 = 0;
	let mut j = height.len() - 1;
	let mut i = 0;
	while j > i {
		let this_area = area(i,j,&height);
		if this_area > max {
			max = this_area;
		}
		
		if height[i] <= height[j] {
			i = i+1;
		} else {
			j = j - 1;
		}
	}
	return max;
}

fn area(i: usize, j: usize, heights: &Vec<i32>) -> i32 {
	let width = (j - i) as i32;
	if heights[i] > heights[j] {
		return heights[j] * width;
	} else {
		return heights[i] * width;
	}
}

fn main() {
	let t1 = vec![1,1,3];
	let res = max_area(t1.clone());
	println!("{:?} -> {}", t1, res);
}