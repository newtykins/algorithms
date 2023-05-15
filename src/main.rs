mod search;
mod sort;

fn main() {
    let a = vec![1,2,3,4,5,6];

    println!("{:?} {:?}", search::binary_search(a.clone(), 6), search::linear_search(a, 5));
}
