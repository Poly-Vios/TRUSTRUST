use std::io;

fn main() {

println!("Enter a note (c,d,e,f,g,a,b):");

 let mut note = String::new();
    io::stdin()
        .read_line(&mut note)
        .expect("Failed to read line");

    let note = note.trim().to_lowercase(); 

    if note == "c" {
        println!("doh");
    } else if note == "d" {
        println!("re");
    } else if note == "e" {
        println!("mi");
    } else if note == "f" {
        println!("fa");
    } else if note == "g" {
        println!("sol");
    } else if note == "a" {
        println!("la");
    } else if note == "b" {
        println!("si");
    }           
}