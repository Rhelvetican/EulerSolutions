use std::io::{stdin, stdout, Write};

pub fn input(msg: &str) -> String {
    print!("{}", msg);
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    stdout().flush().unwrap();
    buf
}
