//Using Rust to find the roots of a quadratic equation

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the coefficient of the x^2 term, a: ");
    io::stdin().read_line(&mut input1).expect("Not a Valid String");
    let a:f32 = input1.trim().parse().expect("Not a valide Number");

    println!("Enter the coefficient of x term, b: ");
    io::stdin().read_line(&mut input2).expect("Not a Valid String");
    let b:f32 = input2.trim().parse().expect("Not a valide Number");

    println!("Enter the constant term / y-intercept, c: ");
    io::stdin().read_line(&mut input3).expect("Not a Valid String");
    let c:f32 = input3.trim().parse().expect("Not a valide Number");

    let discriminant:f32 = (b * b) - (4.0 * a * c);

    if discriminant > 0.0
    {
        println!("There are two distinct roots.")
    }
    else if discriminant == 0.0 {
        println!("There is exactly one real root.");
    }
    else {
        println!("There are no real roots.");
    }
    
// Handling negative discriminant safely
    if discriminant >= 0.0 {
        let first_root = (-b + discriminant.sqrt()) / (2.0 * a);
        let second_root = (-b - discriminant.sqrt()) / (2.0 * a);

        println!("The First root, x₁ = {}", first_root);
        println!("The Second root, x₂ = {}", second_root);
    } 
    else {
        println!("Complex roots detected; no real solutions.");
    }
}