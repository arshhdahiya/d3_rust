// This library is used to take input from the user
use std::io;
// This library is used to generate random number
use rand::Rng;
// this library is used for comparing
use std::cmp::Ordering;
// this library is used for importing colors
use colored::*;

fn main() {
    println!("Welcome to the guessing game !");

    loop{

        // Taking input from the user
        println!("Please enter the value between 1 to 100 :-");
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to take input !");

        // println!("The value you have entered is {}", guess);

        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };


        // generating the random secret number
        let secret_number = rand::thread_rng().gen_range(1..101);
        // println!("The secret random number is : {}", secret_number);

        // comparing the results
        match guess.cmp(&secret_number){
            Ordering::Equal => {println!("{}", "You won !".green());
            break;
            },
            Ordering::Greater => println!("{}","Too big !".red()),
            Ordering::Less => println!("{}","Too small !".red()),
        }

    }

}
