use std::fs::File;
use std::io::Write;

fn main() {
    // Vectors
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star",];
    let stout = vec!["Legend", "Turbo King", "Williams",];
    let non_alcoholic = vec![ "Maltina", "Amstel Malta", "Malta Gold", "Fayrouz",];

    // Create file
    let mut file = File::create("drinks.txt").expect("Could not create file");
    file.write_all("Lager Drinks:\n".as_bytes()).expect("write failed");
    for drink in &lager {
        let list = format!("- {}\n", drink);
        file.write_all(list.as_bytes()).expect("write failed");
    }

    file.write_all("\nStout Drinks:\n".as_bytes()).expect("write failed");
    for drink in &stout {
        let list = format!("- {}\n", drink);
        file.write_all(list.as_bytes()).expect("write failed");
    }

    file.write_all("\nNon-Alcoholic Drinks:\n".as_bytes()).expect("write failed");
    for drink in &non_alcoholic {
        let list = format!("- {}\n", drink);
        file.write_all(list.as_bytes()).expect("write failed");
    }

    println!("File 'drinks.txt' created successfully!");
}