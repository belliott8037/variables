fn main() {
    // mut makes the variable mutable, variables are immutable by default.
    // replace "let" with "const" => keyword to make a constant 
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("Spaces = {}",spaces);
}