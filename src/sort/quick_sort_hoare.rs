use std::cmp;

pub fn sort<T: cmp::Ord>(list: &mut [T], low: usize, high: usize) {
	if high > low {
		let mut i = low;
		let mut j = high;

		loop {
			while list[i] < list[low] { i += 1 }
			while list[j] > list[low] { j -= 1 }

			if i >= j { break }

			list.swap(i, j);

			i += 1;
			j -= 1;
		}
		
		sort(list, low, j);
		sort(list, j + 1, high);
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use rand::{distributions::Uniform, Rng};

	#[test]
	fn quick_sort_hoare_test() {
		// Arrange
		let mut rng = rand::thread_rng();
		let range = Uniform::new(0, 100);
		let mut list : Vec<u64> = (0..100).map(|_| rng.sample(&range)).collect();
		let high = list.len() - 1;

		// Act
		sort(&mut list, 0, high);

		// Assert
		for i in 1..100{
			assert_le!(list[i-1], list[i]);
		}
	}
}