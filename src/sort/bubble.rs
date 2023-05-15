use std::ops::Mul;

pub fn bubble_sort<T: Mul<i32> + Copy>(arr: &mut Vec<T>, asc: bool)
where
    <T as Mul<i32>>::Output: PartialOrd,
{
    let multiplier = if asc { 1 } else { -1 };
    let mut n = arr.len();

    loop {
        for i in 1..=n - 1 {
            if arr[i - 1] * multiplier > arr[i] * multiplier {
                arr.swap(i - 1, i);
            }

            n = i;
        }

        if n <= 1 {
            break;
        }
    }
}
