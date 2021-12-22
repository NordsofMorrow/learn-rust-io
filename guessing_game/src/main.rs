use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {


    let age: Option<i8> = Some(12);

    match age {
        Some(age) => {
            if age >= 18 {
                println!("Can have beer");
            }
            else {
                println!("Can;t have beer, only {}", age)
            }
        },
        None => println!("Uhh")
    }

    println!("Guess the number!");

    let range = 1..101;

    let secret_number = rand::thread_rng().gen_range(range);

    // This variable is shadowing guess above
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
