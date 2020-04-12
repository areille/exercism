pub fn factors(mut n: u64) -> Vec<u64> {
    let mut res = Vec::<u64>::new();
    while n % 2 == 0 {
        res.push(2);
        n /= 2;
    }
    let limit = (n as f64).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        while n % i == 0 {
            res.push(i);
            n /= i;
        }
    }
    if n > 2 {
        res.push(n)
    }
    res
}
