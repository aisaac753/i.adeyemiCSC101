// Rust program to calculate the area of a trinagle given three sides

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter first edge of trinagle: ");
    io::stdin().read_line(&mut input1).expect("Not a vaild string");
    let a:f32 = input1.trim().parse().expect("Not a vaild number");

    println!("Enter second edge of trinagle: ");
    io::stdin().read_line(&mut input2).expect("Not a vaild string");
    let b:f32 = input2.trim().parse().expect("Not a vaild number");

    println!("Enter third edge of trinagle: ");
    io::stdin().read_line(&mut input3).expect("Not a vaild string");
    let c:f32 = input3.trim().parse().expect("Not a vaild number");

    let s:f32 = (a + b + c)/ 2.0;
    let mut area:f32 = s * (s - a) * (s - b) * (s - c);
    area = area.sqrt();

    println!("Area of a trinagle: {}", area);
}
