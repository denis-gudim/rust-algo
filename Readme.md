# Rust algorithm examples

Just another one repository with simple algorithms implementation based on Rust programming language

## Sort

### 1. Bubble sort

```rust,no_run
fn sort<T: Ord>(list: &mut [T]) {
	for _ in 0..list.len(){
		for i in 1..list.len(){
			if list[i-1] > list[i] {
				list.swap(i-1, i);
			}
		}
	}
}
```

### 2.1. Quick sort (Lomuto partition scheme)

```rust,no_run
fn sort<T: Ord>(list: &mut [T], low: usize, high: usize) {
	if high > low {
		let mut i = low;

		for j in low..high {
			if list[j] <= list[high] { 
				list.swap(i, j);
				i += 1;
			}
		}

		list.swap(i, high);

		if i > 0 { sort(list, low, i - 1) }

		sort(list, i + 1, high);
	}
}
```

### 2.2. Quick sort (Hoare partition scheme)

```rust,no_run
fn sort<T: Ord>(list: &mut [T], low: usize, high: usize) {
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
```

### 3. Insertion sort

```rust,no_run
fn sort<T: Ord + Copy>(list: &mut [T]) {
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
```
