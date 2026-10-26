use std::io;

fn main() {
    let mut score = 0;
    loop {
        let number: i32 = rand::random_range(1..=100);
        println!("what do you think the number is");
        loop {
            let mut user_number = String::new();

            io::stdin()
                .read_line(&mut user_number)
                .expect("Failed to read line");

            let user_number: i32 = match user_number.trim().parse() {
                Ok(number) => number,

                Err(_) => {
                    println!("Please enter a valid number!");
                    continue;
                }
            };

            if user_number > number {
                println!("the number is smaller")
            } else if user_number < number {
                println!("the number is bigger")
            } else {
                println!("you guessed the number");
                score += 1;
                break;
            };
        }
        loop {
            println!("You won {score} times!");
            println!("Do you want to continue? Type 1 for yes, 0 for no");

            let mut next = String::new();

            io::stdin()
                .read_line(&mut next)
                .expect("Failed to read line");

            let next: i32 = match next.trim().parse() {
                Ok(number) => number,

                Err(_) => {
                    println!("Please enter a valid number!");
                    continue;
                }
            };

            if next == 1 {
                break;
            } else if next == 0 {
                return;
            } else {
                println!("Please enter 1 or 0!");
            }
        }
    }
}
