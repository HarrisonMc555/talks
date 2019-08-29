% A Brief Introduction to Rust
% Harrison McCullough

# Introduction

## What is Rust?

A programming language focused on:

  - Performance
  - Reliability
  - Productivity


## What makes Rust special?

  - Memory safety without garbage collection
  - Expressive type system
  - Explicit mutability


# Example

## Hello World

```rust
fn main() {
    println!("Hello, World");
}
```


# Benefits

## Mutability is the Exception

```rust
fn main() {
    let count = 5;
    let mut numbers = HashSet::new();
    for x in 0..count {
        numbers.insert(x);
    }
    println!("There are {} numbers",
             numbers.len());
}
```


## No Invalid References

```cpp
// C++
int main() {
    std::vector<std::string> v;
    v.push_back("Hello");
    std::string& x = v[0];
    v.push_back("world");
    std::count << x;
}
```

## No Invalid References

```rust
// Rust
int main() {
    let mut v = vec![];
    v.push("Hello");
    let x = &v[0];
    v.push("world");
    println!("{}", x);
}
```

## No Invalid References

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> use-after-free.rs:6:5
  |
5 |     let x = &v[0];
  |              - immutable borrow occurs here
6 |     v.push("world");
  |     ^^^^^^^^^^^^^^^ mutable borrow occurs here
7 |     println!("{}", x);
  |                    - immutable borrow later used here
```


## No Invalid References

```rust
// Rust
int main() {
    let mut v = vec![]; // v is owner
    v.push("Hello");    // |
    let x = &v[0];      // | x borrows
    v.push("world");    // ** conflict
    println!("{}", x);
}
```
## No Invalid References

> "During the 2 years of development, we have never experienced any
> memory-related bugs like use-after-free or double free."
>
> ⁠— an engineer from Mozilla


# Conclusion

## 

This is all really cool stuff!

<div class="notes">
Cover these things

  - Ownership
  - References
    - Traits
</div>


# Attributions

## Attributions

  - [https://www.slideshare.net/jaejukim9/rust-programming-language](https://www.slideshare.net/jaejukim9/rust-programming-language)
