use std::io;

use rand::Rng;


pub fn play() {
    println!("Guess the type!");
    let my_type = rand::thread_rng().gen_range(1..4);
    
    if my_type == 1 {
        random_integer();
    } else if my_type == 2 {
        random_double();
    } else if my_type == 3 {
        random_character();
    }

}

pub trait AnyExt {
    fn type_name(&self) -> &'static str;
}

impl<T> AnyExt for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

fn random_character() {
    let my_variable = 'd';
    println!("{}",my_variable);
    expectInput(my_variable.type_name())
}

fn random_double() {
    let my_variable = 3.1415;
    println!("{}", my_variable);
    expectInput(my_variable.type_name())
}

fn random_integer() {
    let my_variable = 5;
    println!("{}", my_variable);
    expectInput(my_variable.type_name())}

fn expectInput(variable: &'static str) {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(..) => {
            if input.trim() == variable {
                println!("{}","Great!");
            } else {
                println!("{}","Nope!");
            }
        },
        Err(error) => println!("error: {}", error)
    }
}