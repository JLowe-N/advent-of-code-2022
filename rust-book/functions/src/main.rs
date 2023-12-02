fn main() {
    println!("Hello, world!");

    another_function(5);

    // Demonstrating expressions & return values
    let x = five();

    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("Another function. The value of x is: {x}");
}

fn five() -> i32 {
    5
}

