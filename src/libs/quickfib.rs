pub fn quickfib(n: u64) -> u64 {
    fn inner(n: u64) -> (u64, u64) {
        if n == 0 {
            (0, 1)
        } else {
            let (a, b) = inner(n / 2);
            let c = a * (2 * b - a);
            let d = a * a + b * b;

            if n % 2 == 0 {
                (c, d)
            } else {
                (d, c + d)
            }
        }
    }

    inner(n).0
}
