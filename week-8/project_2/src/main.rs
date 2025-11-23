//Applicant With Highest Experience
use std::io;

fn is_valid_name(name: &str) -> bool {
    for ch in name.chars() {
        if ch.is_alphabetic() || ch == ' ' {
            continue;
        } else {
            return false;
        }
    }
    true
}


fn main() {
    let mut names: Vec<String> = Vec::new();
    let mut years: Vec<u32> = Vec::new();

    //Applicant entry
    println!("How many applicants? ");

    let mut num_input = String::new();
    io::stdin().read_line(&mut num_input).expect("Failed");
    let num: usize = num_input.trim().parse().unwrap_or(0);

    if num == 0 {
        println!("Invalid number!");
        return;
    }

    //Inputs & Validation
    for i in 1..=num {
        println!("\nApplicant {}:", i);

        let mut name = String::new();
        println!("Enter name: ");
        io::stdin().read_line(&mut name).expect("Failed");
        let name = name.trim().to_string();

        if name.is_empty() || !is_valid_name(&name) {
            println!("Invalid name entered!");
            return;
        }

        let mut yrs = String::new();
        println!("Enter years of programming experience (integers): ");
        io::stdin().read_line(&mut yrs).expect("Failed");

        let exp: u32 = yrs.trim().parse().unwrap_or(0);
            if exp == 0 {
        println!("Invalid programming experience!");
        return;
    }

        names.push(name);
        years.push(exp);
    }

    // Find highest experience
    let mut max_index = 0;
    for i in 1..years.len() {
        if years[i] > years[max_index] {
            max_index = i;
        }
    }

    //Report
    println!("\nAPPLICANT WITH HIGHEST EXPERIENCE:");
    println!("Name: {}", names[max_index]);
    println!("Experience: {} years", years[max_index]);
}