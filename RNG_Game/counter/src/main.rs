use std::io;

fn main() {
    let mut counter = 0;
    loop {
        println!("Current count: {}", counter);
        println!("Type '+' to add 1, '-' to subtract 1, or 'q' to quit:");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");
        let user_input = user_input.trim();
        if user_input == "+" {
            counter = counter + 1;
        } else if user_input == "-" {
            counter = counter - 1;
        } else if user_input == "q" {
            println!("EXIT");
            break;
        } else {
            println!("Please type only '+', '-', or 'q'");
        }
    }
}