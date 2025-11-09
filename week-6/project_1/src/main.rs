//Customer's Food Menu Program🍉😋
use std::io;

fn main() {
    // Food Menu
    println!("\nCustomer's Food Menu:\n
        P = Poundo Yam/Edinkaiko Soup  - ₦3,200
        F = Fried Rice & Chicken       - ₦3,000
        A = Amala & Ewedu Soup         - ₦2,500
        E = Eba & Egusi Soup           - ₦2,000
        W = White Rice & Stew          - ₦2,500\n");

    // Inputing order
    let mut food_type = String::new();
    let mut quantity = String::new();

    println!("Dear Customer, enter the food code you would like to order (P, F, A, E, or W): ");
    io::stdin().read_line(&mut food_type).expect("Failed to read input😞");

    println!("Enter the quantity: ");
    io::stdin().read_line(&mut quantity).expect("Failed to read input😞");

    // Parsing quantity as unsigned integer to prevent negatives
    //Unwrap_or ensures my code doesn't break😏😜
    let quantity: u32 = quantity.trim().parse().unwrap_or(0);

    if quantity == 0 {
        println!("Invalid quantity entered!");
        return;
    }

    // Prices
    let p_price = 3200.0;
    let f_price = 3000.0;
    let a_price = 2500.0;
    let e_price = 2000.0;
    let w_price = 2500.0;

    // Converting input to uppercase for consistency
    let food_code = food_type.trim().to_uppercase();

    // Calculating total using if-else
    let total_charge: f32;

    if food_code == "P" {
        total_charge = p_price * quantity as f32;
    } else if food_code == "F" {
        total_charge = f_price * quantity as f32;
    } else if food_code == "A" {
        total_charge = a_price * quantity as f32;
    } else if food_code == "E" {
        total_charge = e_price * quantity as f32;
    } else if food_code == "W" {
        total_charge = w_price * quantity as f32;
    } else {
        println!("Invalid food code entered!");
        return;
    }

    // Appling 5% discount if applicable
    let overall_charge: f32;
    if total_charge > 10_000.0 {
        overall_charge = total_charge * 0.95;
    } else {
        overall_charge = total_charge;
    }

    // Printing receipt
    println!("\n========= Customer's Receipt =========");
    println!("Food Code: {}", food_code);
    println!("Quantity: {}", quantity);
    println!("Total Charge: ₦{:.2}", total_charge);
    println!("Final Charge (after discount if any): ₦{:.2}", overall_charge);
    println!("======================================");
}
