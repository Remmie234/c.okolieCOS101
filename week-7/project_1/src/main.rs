//Calculating the area or volume of different shapes🧮➕
use std::io;

//Equation Functions
fn area_trapezium_formula()->f64 {
    let mut height = String::new();
    let mut base_1 = String::new();
    let mut base_2 = String::new();

    //Input
    println!("Please input the height: ");
    io::stdin().read_line(&mut height).expect("Failed to read input😞");
    let h:f64 = height.trim().parse().unwrap_or(0.0);

    println!("Please input the first base: ");
    io::stdin().read_line(&mut base_1).expect("Failed to read input😞");
    let b1:f64 = base_1.trim().parse().unwrap_or(0.0);

    println!("Please input the second base: ");
    io::stdin().read_line(&mut base_2).expect("Failed to read input😞");
    let b2:f64 = base_2.trim().parse().unwrap_or(0.0); 

    //Checking Validity
    if h <= 0.0 || b1 <= 0.0 || b2 <= 0.0{
        println!("Invalid value entered!😓");
        return 0.0;
    }

    //Calculating Area
    let area:f64 = (h / 2.0) * (b1 + b2);
    return area;
}

fn area_rhombus_formula()->f64 {
    let mut diagonal_1 = String::new();
    let mut diagonal_2 = String::new();

    //Inputs
    println!("Please input the first diagonal: ");
    io::stdin().read_line(&mut diagonal_1).expect("Failed to read input😞");
    let d1:f64 = diagonal_1.trim().parse().unwrap_or(0.0);

    println!("Please input the second diagonal: ");
    io::stdin().read_line(&mut diagonal_2).expect("Failed to read input😞");
    let d2:f64 = diagonal_2.trim().parse().unwrap_or(0.0);

    //Checking Validity
    if d1 <= 0.0 || d2 <= 0.0 {
        println!("Invalid value entered!😓");
        return 0.0;
    }

    //Calculating Area
    let area:f64 = 0.5 * d1 * d2;
    return area;
}

fn area_parallelogram_formula()->f64 {
    let mut base = String::new();
    let mut altitude = String::new();

    //Inputs
    println!("Please input the base: ");
    io::stdin().read_line(&mut base).expect("Failed to read input😞");
    let b:f64 = base.trim().parse().unwrap_or(0.0);

    println!("Please input the altitude: ");
    io::stdin().read_line(&mut altitude).expect("Failed to read input😞");
    let a:f64 = altitude.trim().parse().unwrap_or(0.0);

    //Checking Validity
    if b <= 0.0 || a <= 0.0 {
        println!("Invalid value entered!😓");
        return 0.0;
    }

    //Calculating Area
    let area:f64 = b * a;
    return area;
}

fn area_cube_formula()->f64 {
    let mut length = String::new();

    //Input
    println!("Please input the length of one side: ");
    io::stdin().read_line(&mut length).expect("Failed to read input😞");
    let l:f64 = length.trim().parse().unwrap_or(0.0);

    //Checking Validity
    if l <= 0.0 {
        println!("Invalid value entered!😓");
        return 0.0;
    }

    //Calculating Area
    let area:f64 = 6.0 * l * l;
    return area;
}

fn pi() ->f64 {
    22.0 / 7.0
}

fn volume_cylinder_formula()->f64 {
    let mut radius = String::new();
    let mut height = String::new();

    //Inputs
    println!("Please input the radius: ");
    io::stdin().read_line(&mut radius).expect("Failed to read input😞");
    let r:f64 = radius.trim().parse().unwrap_or(0.0);

    println!("Please input the height: ");
    io::stdin().read_line(&mut height).expect("Failed to read input😞");
    let he:f64 = height.trim().parse().unwrap_or(0.0);

    //Checking Validity
    if r <= 0.0 || he <= 0.0 {
        println!("Invalid value entered!😓");
        return 0.0;
    }

    //Calculating Volume
    let volume:f64 = pi() * r * r * he;
    return volume;
}

//Main Function
fn main() {
    //Greetings
    let mut name = String::new();
    println!("Welcome! I greet you, Honored One!");
    println!("Dear Honored One, please enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read input😞");
    let customer = name.trim();

    //Greetings with name
    println!("Once again, I welcome you, {}", customer);

    //Equations
    println!("Here are the available list of Equations: 🧮➕\n
            A = Area of Trapezium
            B = Area of Rhombus
            C = Area of Parallelogram
            D = Area of Cube
            E = Volume of Cylinder\n");

    //Inputs
    let mut input = String::new();
    println!("Dear {}, please enter the equation code you would like to use (A, B, C, D, E): ", customer);
    io::stdin().read_line(&mut input).expect("Failed to read input😞");

    // Converting input to uppercase for consistency
    let equation_code = input.trim().to_uppercase();

    let result:f64;

    //Invoking Equation function
    if equation_code == "A" {
        result = area_trapezium_formula();
    } else if equation_code == "B" {
        result = area_rhombus_formula();
    } else if equation_code == "C" {
        result = area_parallelogram_formula();
    } else if equation_code == "D" {
        result = area_cube_formula();
    } else if equation_code == "E" {
        result = volume_cylinder_formula();
    } else {
        println!("Invalid code entered!😓");
        return;
    }

    if result == 0.0 {
        println!("Dear {}, the calculation failed due to invalid input😞", customer);
    } else {
        println!("Dear {}, the result is {} ✌🏼😁", customer, result);
    }
}