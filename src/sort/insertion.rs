use std::ops::Mul;

pub fn insertion_sort<T: Mul<i32> + Copy>(arr: &mut Vec<T>, asc: bool)
where
    <T as Mul<i32>>::Output: PartialOrd,
{
    let multiplier = if asc { 1 } else { -1 };
    let mut i = 1;

    while i < arr.len() {
        let x = arr[i];
        let mut j = i - 1;

        while j > 0 && arr[j] * multiplier > x * multiplier {
            arr[j + 1] = arr[j];
            j -= 1;
        }

        arr[j + 1] = x;
        i += 1;
    }
}
