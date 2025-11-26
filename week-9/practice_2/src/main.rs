use std::io::Read;
use std::io::Write;
use std::fs::File;


fn main() {
    let mut file = File::create("mycreated text").expect("write failed");
    file.write_all("Xelex Community\n".as_bytes()).expect("write failed");
    file.write_all("You should check it out on Youtube, you know\n".as_bytes()).expect("write failed");
    file.write_all("The Subscribe and Like all Shorts, teehee".as_bytes()).expect("write failed");

    let mut file = File::open("mycreated text").expect("open failed");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("read failed");
    println!("{}", contents);
}