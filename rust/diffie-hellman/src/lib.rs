use rand::{thread_rng, Rng};

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1;
    base %= m;
    while exp > 0 {
        if exp % 2 == 1 {
            r = (r * base) % m;
        }
        exp >>= 1;
        base = base.pow(2) % m;
    }
    r
}

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(b_pub, a, p)
}
