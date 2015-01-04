#![feature(slicing_syntax)]

use std::vec::Vec;

// Split a slice into N mutable slices
// Currently only possible with recursive calls to split_at
pub enum SplitErr<'a> {
	TooMany(&'a str),
	Zero(&'a str),
}

pub fn split<'s, 'e, T: 's>(input: &'s[T], n: uint) -> Result<Vec<&'s [T]>, SplitErr<'e>> {
	if n > input.len() {
		return Err(SplitErr::TooMany("Cannot split when n > len"));
	}

	if n == 0 {
		return Err(SplitErr::Zero("Cannot split into 0 slices"));
	}

	let dist = calc_dist(input.len(), n);

	let mut output: Vec<&[T]> = Vec::with_capacity(dist.len());

	recursive(input, &mut output, dist.as_slice());

	Ok(output)
}

//fn split_mut

fn recursive<'s, T: 's>(input: &'s[T], output: &mut Vec<&'s[T]>, dist: &[uint]) {
	let (x, y) = input.split_at(dist[0]);

	(*output).push(x);

	if dist.len() > 1 {
		recursive(y, output, dist[1..dist.len()]);
	}
}

//fn recursive_mut

fn calc_dist(count: uint, num_bins: uint) -> Vec<uint> {
	if num_bins > count {
		return Vec::with_capacity(0);
	}

	let mut bins: Vec<uint> = Vec::with_capacity(num_bins);

	for _ in (0..num_bins) {
		bins.push(count / num_bins);
		println!("push {}", count / num_bins);
	}

	{
		let mut bin_iter = bins.as_mut_slice().iter_mut();

		// Evenly distribute remainder over all bins
		for _ in (0..count % num_bins) {
			(*bin_iter.next().unwrap()) += 1;
			println!("add 1")
		}
	}

	bins
}

#[cfg(test)]
mod test {
	use super::{calc_dist, recursive, split};

	#[test]
	fn test_dist() {
		assert_eq!(calc_dist(2, 8),  vec![]);
		assert_eq!(calc_dist(1, 1),  vec![1u]);
		assert_eq!(calc_dist(10, 3), vec![4u,3,3]);
		assert_eq!(calc_dist(10, 8), vec![2u,2,1,1,1,1,1,1]);
		assert_eq!(calc_dist(5, 2),  vec![3u,2]);
		assert_eq!(calc_dist(5, 5),  vec![1u,1,1,1,1]);
	}

	#[test]
	fn test_recursive() {
		let input: Vec<uint> = vec![10, 10];
		let mut out: Vec<&[uint]> = Vec::new();

		recursive(input.as_slice(), &mut out, vec![1,1].as_slice());

		for s in out.iter() {
			for (j, n) in s.iter().enumerate() {
				assert_eq!(j, 0); // Slice has 1 element
				assert_eq!(*n, 10);
			}
		}
	}

	#[test]
	fn test_split() {
		let input: Vec<uint> = vec![1,2,3,4];
		let test = input.as_slice();

		match split(input.as_slice(), 2) {
			Ok(v)  => assert_eq!(v, vec![test[0..2], test[2..4]]),
			Err(e) => panic!(e),
		}
	}
}
