use std::fs::File;
use std::io::Write;

fn main() {
    // Arrays / Vectors for student data
    let names = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemonh"];
    let matric = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let dept = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let level = vec![300, 100, 200, 200, 100];

    // Create
    let mut file = File::create("students.txt").expect("Cannot create file");
    file.write_all("PAU SMIS STUDENT RECORDS\n\n".as_bytes()).expect("Cannot write to file");
    file.write_all("NAME | MATRIC | DEPARTMENT | LEVEL\n".as_bytes()).expect("Cannot write to file");

    // Loop through students
    for i in 0..names.len() {
        let list = format!(
            "{} | {} | {} | {}\n",
            names[i], matric[i], dept[i], level[i]
        );

        file.write_all(list.as_bytes()).expect("Cannot write to file");
    }

    println!("File 'students.txt' created successfully");
}
