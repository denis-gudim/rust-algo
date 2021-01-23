use std::cmp;

pub fn sort<T: cmp::Ord>(list: &mut [T]) {
	for _ in 0..list.len(){
		for i in 1..list.len(){
			if list[i-1] > list[i] {
				list.swap(i-1, i);
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand::{distributions::Uniform, Rng};

	#[test]
	fn bubble_sort_test() {
		// Arrange
		let mut rng = rand::thread_rng();
		let range = Uniform::new(0, 100);
		let mut list : Vec<u64> = (0..100).map(|_| rng.sample(&range)).collect();

		// Act
		sort(&mut list);

		// Assert
		for i in 1..100{
			assert_le!(list[i-1], list[i]);
		}
	}
}