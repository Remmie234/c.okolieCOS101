//Rust program + Postgres Database

use postgres::{Client, NoTls};
use std::io;


//Connect to Database
fn connect_db() -> Client {
    Client::connect(
        "host=localhost user=postgres password=cos101 dbname=globacom_dbase",
        NoTls,
    ).expect("Failed to connect to database")
}

//Salutation
fn greetings(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to Read.");

    format!("Once again, greetings {}", input.trim())   
}

//Input
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to Read.");
    input.trim().to_string()
}

fn main() {
    let welcome_message = greetings("Greetings, please input your name:");
    println!("{}\n", welcome_message);
    let role = get_input("Enter role (admin/project_manager/employee/customer/vendor):");
    let password = get_input("Enter password:");

    if password != "cos101" {
        println!("Access denied ❌");
        return;
    }

    let mut client = connect_db();

    if role == "admin" {
        show_all_tables(&mut client);
    } else if role == "project_manager" {
        show_project(&mut client);
    } else if role == "employee" {
        show_staff(&mut client);
    } else if role == "customer" {
        show_customer(&mut client);
    } else if role == "vendor" {
        show_dataplan(&mut client);
    } else {
        println!("Invalid role ❌");
    }
}


//Functions
fn show_all_tables(client: &mut Client) {
    println!("Admin access granted ✅");

    show_project(client);
    show_staff(client);
    show_customer(client);
    show_dataplan(client);
    show_department(client);
}

fn show_project(client: &mut Client) {
    println!("Project_manager access granted ✅");
    let rows = client
        .query("SELECT pno, pname, pduration::Text, project_managerid FROM project", &[])
        .expect("Project Query Failed.");

    println!("{:<10} | {:<15} | {:<20} | {:<10}", "PNO", "PNAME", "PDURATION", "PROJECT_MANAGERID");
    println!("{:-<71}", "");

    for row in rows {
        let pno: i32 = row.get(0);
        let pname: String = row.get(1);
        let pduration: String = row.get(2);
        let project_managerid: i32 = row.get(3);

        println!("{:<10} | {:<15} | {:<20} | {:<10}", pno, pname, pduration, project_managerid);
    }
    println!("\n");
}

fn show_staff(client: &mut Client) {
    println!("Staff access granted ✅");
    let rows = client
        .query("SELECT staff_id, staff_name, dno, staff_sal, age, mobile FROM staff", &[])
        .expect("Staff Query Failed.");

    println!("{:<10} | {:<20} | {:<5} | {:<10} | {:<5} | {:<15}", "STAFF_ID", "STAFF_NAME", "DNO", "STAFF_SAL", "AGE", "MOBILE");
    println!("{:-<75}", "");

    for row in rows {
        let staff_id: i32 = row.get(0);
        let staff_name: String = row.get(1);
        let dno: i32 = row.get(2);
        let staff_sal: f32 = row.get(3);
        let age: i16 = row.get(4);
        let mobile: String = row.get(5);

        println!("{:<10} | {:<20} | {:<5} | {:<10} | {:<5} | {:<15}", staff_id, staff_name, dno, staff_sal, age, mobile);
    }
    println!("\n");
}

fn show_customer(client: &mut Client) {
    println!("Customer access granted ✅");
    let rows = client
        .query("SELECT c_id, c_name, c_age, c_email, c_mobile, eid, data_id FROM customer", &[])
        .expect("Customer Query Failed.");

    println!("{:<5} | {:<15} | {:<5} | {:<20} | {:<15} | {:<5} | {:<5}", "C_ID", "C_NAME", "C_AGE", "C_EMAIL", "C_MOBILE", "EID", "DATA_ID");
    println!("{:-<90}", "");

    for row in rows {
        let c_id: i32 = row.get(0);
        let c_name: String = row.get(1);
        let c_age: i32 = row.get(2);
        let c_email: String = row.get(3);
        let c_mobile: String = row.get(4);
        let eid: i32 = row.get(5);
        let data_id: i32 = row.get(6);

        println!("{:<5} | {:<15} | {:<5} | {:<20} | {:<15} | {:<5} | {:<5}", c_id, c_name, c_age, c_email, c_mobile, eid, data_id);
    }
    println!("\n");
}

fn show_dataplan(client: &mut Client) {
    println!("Vendor access granted ✅");
    let rows = client
        .query("SELECT data_id, data_size, data_duration_in_days, data_price_in_naira FROM dataplan", &[])
        .expect("Dataplan Query Failed.");

    println!("{:<10} | {:<15} | {:<20} | {:<10}", "DATA_ID", "DATA_SIZE", "DATA_DURATION (DAYS)", "DATA_PRICE (NAIRA)");
    println!("{:-<72}", "");

    for row in rows {
        let data_id: i32 = row.get(0);
        let data_size: String = row.get(1);
        let data_duration_in_days: i32 = row.get(2);
        let data_price_in_naira: f32 = row.get(3);

        println!("{:<10} | {:<15} | {:<20} | {:<10}", data_id, data_size, data_duration_in_days, data_price_in_naira);
    }
    println!("\n");
}

fn show_department(client: &mut Client) {
    println!("Department access granted ✅");
    
    let rows = client
        .query("SELECT dept_managerid, dno, dname, dlocation, pno FROM department", &[])
        .expect("Department Query Failed");

    println!("{:<10} | {:<5} | {:<15} | {:<15} | {:<5}", "MGR_ID", "DNO", "DNAME", "LOCATION", "PNO");
    println!("{:-<65}", "");

    for row in rows {
        let dept_managerid: i32 = row.get(0); 
        let dno: i32 = row.get(1);            
        let dname: String = row.get(2);      
        let dlocation: String = row.get(3);  
        let pno: i32 = row.get(4);

        println!(
            "{:<10} | {:<5} | {:<15} | {:<15} | {:<5}", dept_managerid, dno, dname, dlocation, pno);
    }
}