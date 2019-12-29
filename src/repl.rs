use std::io::{self, Write};

pub fn run() {
    let prompt = ">> ";
    let mut scan = String::new();

    io::stdout().flush().unwrap();

    loop {
        io::stdin().read_line(&mut scan).expect("failed to read line");

    }
}
