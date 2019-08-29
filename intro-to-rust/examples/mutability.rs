use std::collections::HashSet;

fn main() {
    let count = 5;
    let mut numbers = HashSet::new();
    for x in 0..count {
        numbers.insert(x);
    }
    println!(
        "There are {} numbers",
        numbers.len()
    );
}
