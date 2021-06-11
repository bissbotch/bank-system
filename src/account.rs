use std::io;

pub fn login() {
    println!("Enter your username: \n");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username);

    println!("Enter your password: \n");
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password);
}

pub fn register() {
    println!("Enter your username: \n");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username);

    println!("Enter your password: \n");
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password);
}