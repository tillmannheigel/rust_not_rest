use rand::Rng;

pub fn play() {
    println!("Guess the type!");
    let my_type = rand::thread_rng().gen_range(1..3);
    
    if my_type == 1 {
        random_integer();
    } else if my_type == 2 {
        random_double();
    } else if my_type == 3 {
        random_character();
    }

}

fn random_character() {
    todo!()
}

fn random_double() {
    todo!()
}

fn random_integer() {
    todo!()
}