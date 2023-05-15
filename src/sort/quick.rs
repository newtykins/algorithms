use std::ops::Mul;

fn partition<T: PartialOrd + Mul<i32> + Copy>(
    arr: &mut Vec<T>,
    asc: bool,
    left: isize,
    right: isize,
) -> isize
where
    <T as Mul<i32>>::Output: PartialOrd,
{
    let multiplier = if asc { 1 } else { -1 };
    let pivot = right;
    let mut i = left - 1;

    for j in left..=right - 1 {
        if arr[j as usize] * multiplier <= arr[pivot as usize] * multiplier {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, pivot as usize);

    i + 1
}

fn _quicksort<T: PartialOrd + Mul<i32> + Copy>(arr: &mut Vec<T>, asc: bool, left: isize, right: isize)
where
    <T as Mul<i32>>::Output: PartialOrd,
{
    if left <= right {
        let p = partition(arr, asc, 0, right);

        _quicksort(arr, asc, left, p - 1);
        _quicksort(arr, asc, p + 1, right);
    }
}

pub fn quicksort<T: PartialOrd + Mul<i32> + Copy>(arr: &mut Vec<T>, asc: bool)
where
    <T as Mul<i32>>::Output: PartialOrd,
{
    _quicksort(arr, asc, 0, (arr.len() - 1) as isize)
}
