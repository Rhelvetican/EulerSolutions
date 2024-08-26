use utils::get_id;

mod libs;
mod solutions;

use paste::paste;
#[macro_use]
mod utils;

fn main() {
    let mut id = get_id();
    while id != 99999 {
        match_id!(id, 1, 2);
        id = get_id();
    }
}
