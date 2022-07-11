fn max_area(height: Vec<i32>) -> i32 {
	let mut max : i32 = 0;
	let n : usize = height.len();
	for i in 0..n {
		let max_width = (n - 1 - i) as i32;
		if height[i] * max_width <= max {
			continue;
		}
		for j in (i+1)..n {
			let this_area = area(i,j,&height);
			if this_area > max {
				max = this_area;
			}
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