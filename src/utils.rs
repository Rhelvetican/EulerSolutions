#[macro_export]
macro_rules! match_id {
    ($obj:expr, $($id:literal), *) => {
        match $obj {
            $($id => println!("{}", paste!(solutions::[<p $id>]::solution()))),*,
            _ => println!("Unimplemented problem."),
        }
    };
}

use std::io::{stdin, stdout, Write};

fn input(msg: &str) -> String {
    print!("{}", msg);
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    stdout().flush().unwrap();
    buf
}

pub fn get_id() -> usize {
    let mut res = input("Input problem ID: ").trim().parse::<usize>();
    while res.is_err() {
        res = input("Input problem ID: ").trim().parse::<usize>();
    }

    if let Ok(res) = res {
        return res;
    }

    0
}
