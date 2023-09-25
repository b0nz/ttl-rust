use std::io;

fn main() {
    let mut name = String::new();
    print!("Input your name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Hello, {}!", name);
}
