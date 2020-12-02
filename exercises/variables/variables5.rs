// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)


fn main() {
    let mut number = "T-H-R-E-E";
    println!("Spell a Number : {}", number);
    let number = 3; // Used shadowing to create a new variable with the same name.
    // This ensures we still have an immutable variable, but can still use the same name even though its a different type
    // Note: can not use mut for this solution as you can't change types https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
    println!("Number plus two is : {}", number + 2);
}
