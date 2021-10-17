use std::io;
use std::io::Write;

mod guess;
mod type_writer;

fn main() {
    println!("What game do you want to play?");
    println!("1 - Guessing Game");
    println!("2 - Type Writer");
    print!("> ");
    io::stdout().flush().expect("flush");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(..) => {
            println!("{}", input);
            if input.contains("1") {
                guess::play();
            } else if input.contains("2") {
                type_writer::play()
            }
        }
        Err(error) => println!("error: {}", error),
    }
}
