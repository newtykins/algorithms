pub fn binary_search<T: PartialOrd>(arr: &Vec<T>, target: T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low != high {
        let midpoint = (low + high + 1) / 2;

        if arr[midpoint] > target {
            high = midpoint - 1;
        } else {
            low = midpoint;
        }
    }

    if arr[low] == target {
        Some(low)
    } else {
        None
    }
}
