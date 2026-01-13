/*
Using this main.rs to learn some basics of Rust


guessingGame.....

*/

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() 
{
    println!("Welcome to the Guessing Game!");
    
    let secret_number = generate_random_number(1, 100); 
    
   // println!("The secret number is: {}", secret_number);


    loop
    {
          
        println!("Try to guess the number between 1 and 100!");
               
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        if guess.trim() == "exit"
        {
            println!("Exiting the game. Goodbye!");
            break;
        }
                
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };


        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {

                println!("Congratulations! You guessed the correct number!");
                break;
            }
                
        }
        
            
    }

       
}
   








// make random number

fn generate_random_number(min: u32, max: u32) -> u32
{
    let number = rand::rng().random_range(min..=max);

    number


}
