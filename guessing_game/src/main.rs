use std::io; //includes the std:io library
use rand::Rng; //include the Rng trait from the rand crate
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!"); //runs println macro using !

    let random_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess."); //runs println macro using !


    
        let mut guess = String::new(); //let creates variable and mut makes the variable mutable
        
    
        //line breaks are fine as ; is used to end line
        io::stdin()
            .read_line(&mut guess) //using & to reference guess as our input buffer without having to copy or create it
            .expect("Failed to read line"); //read_line returns a io::Result value which has a .expect method we can call
            //if reading input fails it will return an Err value and if expect receives an Err it prints
            //our error and throws a crash
    
        println!("You guessed: {}", guess); //we can do string formatting like python and pass the value of our guess
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("An error occured parsing your input!\n");
                continue;
            }
        };
        
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Guess matches the secret number!");
                break;
            }
        }
    }
    
}