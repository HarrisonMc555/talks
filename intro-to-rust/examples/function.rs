fn inc(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 5;
    let y = inc(x);
    println!("x is {}, y is {}", x, y);
}
