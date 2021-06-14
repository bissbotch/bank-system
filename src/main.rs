mod account;
use std::io;
fn main() {
    dashboard();
}

fn dashboard() {
    println!("Welcome to the ATM");
    println!("Do you have an account? y/n");
    let mut choice = String::new();
    io::stdin()
    .read_line(&mut choice)
    .expect("Error getting input");

    if choice == "y" {
        use account::login;
        login();
    }

    else if choice == "n" {
        use account::register;
        register();
    }
    
    else {
        println!("Please enter a valid option\n");
    }
}

