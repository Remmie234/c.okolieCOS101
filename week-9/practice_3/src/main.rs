use std::fs::remove_file;
use std::io::Write;
use std::fs::File;

fn main() {
    let mut file = File::create("tired.txt").expect("create failed");
    file.write_all("I need an assistant".as_bytes()).expect("write failed");
    file.write_all("Who is obviously skilled at editing, duh".as_bytes()).expect("write failed");

    remove_file("tired.txt").expect("removing file failed");
    println!("Tired.txt file removed successfully");
}