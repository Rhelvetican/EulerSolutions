use crate::libs::quickfib::quickfib;

pub fn solution() -> u64 {
    (1..34).map(quickfib).filter(|x| x % 2 == 0).sum()
}
