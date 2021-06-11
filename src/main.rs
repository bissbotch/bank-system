mod account;
use std::io::{self, Stdin};
fn main() {
    dashboard();
}

fn dashboard() {
    println!("Welcome to the ATM");
    println!("Do you have an account? (1 = Y | 2 = N");
    let mut choice = String::new();
    io::stdin()
    .read_line(&mut choice);

    if choice == "1" {
        login();
    }

    else if choice == "2" {
        register();
    }
    
    else {
        println!("Please enter a valid option\n");
    }
}

