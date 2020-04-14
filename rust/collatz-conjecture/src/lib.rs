fn is_even(number: u64) -> bool {
    number % 2 == 0
}

pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut count = 0;
    while n != 1 {
        if is_even(n) {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        count += 1;
    }
    Some(count)
}
