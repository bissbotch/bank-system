use std::io;
use std::fs::File;

struct User {
    name: String,
    pass: String,
}

pub fn login() {
    println!("Enter your username: \n");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Error reading username");

    println!("Enter your password: \n");
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Error reading password");
}


impl User {
    pub fn register() {
        println!("Enter your username: \n");
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Error reading username");
    
        println!("Enter your password: \n");
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Error reading password");

    }
}

pub fn checkForAccount() {
    let file = File::open("accounts.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error opening the file: {:?}", error),
    };
    
}