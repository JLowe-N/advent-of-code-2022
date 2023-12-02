use std::io;

fn main() {
    // mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constants
    const PI : f64 = 3.14;

    // shadowing
    let y = 5;

    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // Array usage
    let a =[1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize  = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    
    // intentional demonstration
    // This will panic if index exceeds length
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
