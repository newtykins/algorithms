pub fn linear_search<T: PartialEq>(arr: Vec<T>, target: T) -> Option<usize> {
	for (i, item) in arr.iter().enumerate() {
		if item == &target {
			return Some(i);
		}
	}
	
	None
}
