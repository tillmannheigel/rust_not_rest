use std::io;

mod guess;

fn main() {
    println!("What game do you want to play?");
    println!("1 - Guessing Game");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(..) => {
            println!("{}", input);
            if input.eq("1") {
                guess::play();
            }
        }
        Err(error) => println!("error: {}", error),
    }
}
