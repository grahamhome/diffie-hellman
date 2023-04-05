use num_bigint::BigUint;
use rand::{thread_rng, Rng};

mod tests;

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p).unwrap()
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p).unwrap()
}

/// A memory-efficient modular exponentiation method.
fn modular_pow(base: u64, mut exponent: u64, modulus: u64) -> Option<u64> {
    if modulus == 1 {
        return Some(0);
    } else {
        let mut base = BigUint::from(base);
        let mut result: u64 = 1;
        base = base % modulus;
        while exponent > 0 {
            if exponent % 2 == 1 {
                result = u64::try_from((result * base.clone()) % modulus).unwrap();
            }
            exponent = exponent >> 1;
            base = base.pow(2) % modulus;
        }
        return Some(result);
    }
}
