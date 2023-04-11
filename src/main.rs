use std::time::{Instant, Duration}; // Headers for counting the program's running time
use std::io::{stdin, stdout, Write}; // Headers for input and output information

struct Account { // Structure for storing information about the player and his account
    login: String,
    password: String,
    nick_name: String,
    age: u8,
    lvl: u8
}

fn main() {
    let start: Instant = Instant::now(); // Starts the program timer

    loop { // Main program loop
        let mut command: String = String::new(); // Variable for the command entered by the user
        print!("Enter command: ");
        stdout().flush().unwrap(); // Allows you to display the previous line without moving to a new line
        stdin().read_line(&mut command).unwrap(); // Reads the user's command into the 'command' variable

        match command.as_str().trim() {
            "exit" => break, // If the command is 'exit', the program ends
            "reg" => todo!("Register function"),
            "login" => todo!("Login function"),
            "info" => todo!("Info about account function"),
            "logout" => todo!("Logout function"),
            _ => println!("Unknown command") // If the user's command does not match the above, prints 'unknown command'
        }
    }

    let duration: Duration = start.elapsed(); // Reads data from the program timer
    println!("\nThe program ran for {:?}", duration); // Prints data from the program timer
}
