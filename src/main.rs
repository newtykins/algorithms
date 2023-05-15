mod search;
mod sort;

fn main() {
    let mut a = vec![1, 2, 5, 4, 2, 3];
    sort::insertion_sort(&mut a, true);

    println!(
        "{:?} {:?} {:?}",
        a.clone(),
        search::binary_search(&a, 5),
        search::linear_search(&a, 2)
    );
}
