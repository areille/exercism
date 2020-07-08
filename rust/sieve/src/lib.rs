pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound <= 1 {
        return vec![];
    }
    let mut bools = Vec::<bool>::new();
    for _ in 0..upper_bound - 1 {
        bools.push(true);
    }

    let limit = (upper_bound as f64).sqrt() as usize;
    for i in 0..limit - 1 {
        if bools[i] {
            for j in (u64::pow(i as u64 + 2, 2)..upper_bound + 1).step_by(i + 2) {
                bools[j as usize - 2] = false;
            }
        }
    }
    bools
        .iter()
        .enumerate()
        .map(|(i, x)| if *x { i as u64 + 2 } else { 0 })
        .filter(|x| *x != 0)
        .collect()
}
