struct Solution;

impl Solution {
	// Strategy - build on the two sum solution.
	// first you gotta sort the thing. Then we need to split it into
	// a POSITIVE slice, ZERO slice, and a NEGATIVE slice.
	//
	// For solutions including a zero, either all three are zeros
	// (we will have to nCr the ZERO slice which is going to suck) or
	// the solution is see if an NEGATIVE[j] = -POSITIVE[k]
	// And then add a solution for each ZERO[i]
	//    * actually it looks like they don't want duplicate triplets.
	//      so just add a single [0,0,0] is ok if ZEROS.len() > 3
	//
	// For non-zero solutions:
	//
	// For every candiate i in the NEGATIVE slice, we know that j and k must be
	// positive and add up to -i . We can do two sum algorithm on the POSITIVE slice.
	//
	// For every candiate i in the POSITIVE slice...vice versa.
	//
	// A stipulation is that we avoid duplicates. I don't know if we might get some
	// for an case with multiples of the same number where we have e.g. 1, 2, 2, 3
	// when searching for a two-sum we might push a result with the 2 twice?
	// or push (x, 2) and (2, x)
	// these are good cases to check for.

	pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
	}
}