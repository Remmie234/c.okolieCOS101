use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;

fn main() {
    let mut file = File::create("Remmie's File.txt").expect("create failed");
    file.write_all("Hehe, u wanna know a secret🙂\n".as_bytes()).expect("write failed");
    file.write_all("Fine I'll tell u🤔, u unsatiable one, hehe, I like this😉\n".as_bytes()).expect("write failed");
    file.write_all("That secret is...\n".as_bytes()).expect("write failed");

    let mut file = OpenOptions::new().append(true).open("Remmie's File.txt").expect("append failed");
    file.write_all("I am actually a billionaire🤑🫰🏼💰💸\n".as_bytes()).expect("write failed");
    file.write_all("Was that shocking, hehe, caught u had.😉\n".as_bytes()).expect("write failed");
    println!("Append successfully");


































}
