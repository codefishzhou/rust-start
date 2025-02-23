use std::io;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    println!("Please input your guess.");
    
    // for i in 0..100{
    //     let secret_number = rand::rng().random_range(1..=100);
    //     println!("The secret number is: {}", secret_number);
    // }

    for i in 0..20{
        let randon_num = rand::thread_rng().gen_range(1..=100);
        println!("The random number is: {}", randon_num);
    }
}