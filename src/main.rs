use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number!");

    let generated_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = guess.trim().parse().expect("Expect a number!");

        match guess.cmp(&generated_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Equal => {
                println!("YES, you found it!");
                break;
            },
            Ordering::Greater => println!("too big!")
        }

        println!("You guessed: {}", guess);
    }
}
