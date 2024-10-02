fn main() {
    let x = 10;
    let y = 20; // Clippy will warn about `y` being unused
    let sum = x + 5;

    let value: Option<i32> = None;
        
    // Clippy will warn about using `unwrap()` on an `Option` that can be `None`.
    let z = value.unwrap();  // This will panic if `value` is `None`
    

    println!("The sum is: {}", sum);
    println!("{}", z);

}