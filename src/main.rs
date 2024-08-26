use paste::paste;
use utils::input;

mod solutions;
mod utils;

macro_rules! match_id {
    ($obj:expr, $($id:literal), *) => {
        match $obj {
            $($id => println!("{}", paste!(solutions::[<p $id>]::solution()))),*,
            _ => println!("Unimplemented problem."),
        }
    };
}

fn main() {
    let mut id = input("Input problem ID: ").parse::<usize>().unwrap();
    while id != 99999 {
        match_id!(id, 1);
        id = input("Input problem ID: ").parse::<usize>().unwrap();
    }
}
