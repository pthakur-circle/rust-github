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

// Tarpaulin
/// A simple function that adds two numbers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    // #[test]
    // fn test_failure() {
    //     assert_eq!(add(1, 1), 3); // This test will fail
    // }
}