pub fn sort<T: Ord + Copy>(list: &mut [T]) {
	for i in 1..list.len(){
		let mut j = i;
		let tmp = list[i];

		while j > 0 && list[j - 1] > tmp {
			list[j] = list[j - 1];
			j -= 1;
		}

		list[j] = tmp;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand::{distributions::Uniform, Rng};

	#[test]
	fn insertion_sort_test() {
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