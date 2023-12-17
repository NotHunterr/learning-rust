use std::io;


fn main() {
    println!("Please enter your first number: ");
    let mut num_one = String::new();

    io::stdin()
        .read_line(&mut num_one)
        .expect("Invalid input");

    let num_one_int: i32 = num_one.trim().parse().unwrap();
    println!("Please enter your second number: ");
    let mut num_two = String::new();

    io::stdin()
        .read_line(&mut num_two)
        .expect("Invalid input");

    let num_two_int: i32 = num_two.trim().parse().unwrap();
    add_num(num_one_int, num_two_int)
}

fn add_num(x: i32, y: i32) {
    println!("the sum is {}", x + y)
}