fn main() {
    let string: &str = "MAM";
    let length = string.len();
    for loop_1 in 0..length {
        if &string.as_bytes()[loop_1] != &string.as_bytes()[length - loop_1 - 1] {
            println!("This is not an example of Palindrome");
            break;
        }
        else {
            println!(" This is an example of Palindrome");
            break;
        }
    }
}