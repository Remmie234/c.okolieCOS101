//Determining annual incentive

use std::io;

fn main() 
{
    let mut input_name = String::new();
    let mut input_experience = String::new();
    let mut input_age = String::new();


    // input name
    println!("Employee, please enter your name: ");
    io::stdin().read_line(&mut input_name).expect("Failed to read input");
    let name = input_name.trim();

    //input experience
    println!("Employee {}, are you experienced (yes / no): ", name);
    io::stdin().read_line(&mut input_experience).expect("Failed to read input");
    let experience = input_experience.trim().to_lowercase();

    //input age
    println!("Employee {}, please enter your age (in integer): ", name);
    io::stdin().read_line(&mut input_age).expect("Failed to read input");
    let age:u32 = input_age.trim().parse().expect("Please enter a valid input");

    if experience == "yes" && age >= 40
    {
        println!("Employee {}, your incentive is N1,560,000.00", name);
    }
    else if experience == "yes" && age >= 30 && age < 40
    {
    println!("Employee {}, your incentive should be N1,480,000.00", name);
    }
    else if experience == "yes" && age < 28
    {
    println!("Employee {}, your incentive should be N1,300,000.00", name);
    }
    else {
        println!("Employee {}, your incentive should be N100,000.00", name);
    }

}