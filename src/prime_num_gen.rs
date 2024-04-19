use super::*;

fn n_bit_random_biguint(n: u32) -> BigUint {
    let mut rng = rand::thread_rng();

    let min = (BigUint::from(2u32).pow(n - 1)) + 1u32;
    let max = (BigUint::from(2u32).pow(n)) - 1u32;
    rng.gen_biguint_range(&min, &max)
}

fn gen_low_level_prime() -> BigUint {
    let first_primes = sieve_of_eratosthenes::first_primes();

    loop {
        let prime_candidate = n_bit_random_biguint(PRIME_SIZE);

        for divisor in first_primes.iter() {
            if &prime_candidate % divisor == BigUint::from(0u32) {
                break;
            }
        }
        return prime_candidate;
    }
}

fn is_miller_rabin_prime(n: &BigUint, iterations: u32) -> bool {
    let mut rng = rand::thread_rng();

    let n_minus_one = n - 1u32;
    let mut s: u32 = 0;
    let mut d = n_minus_one.clone();

    while &d % 2u32 == BigUint::from(0u32) {
        d >>= 1;
        s += 1;
    }

    let mut y: BigUint;

    'outer: for _ in 0..iterations {
        let a = rng.gen_biguint_range(&BigUint::from(2u32), &n_minus_one);
        let mut x = a.modpow(&d, n);

        if x != BigUint::from(1u32) && x != n_minus_one {
            for _ in 0..s {
                y = x.modpow(&BigUint::from(2u32), n);
                if y == n_minus_one {
                    continue 'outer;
                }
                x = y.clone();
            }
            return false;
        }
    }

    true
}

pub fn find_prime() -> BigUint {
    let prime_candidate = gen_low_level_prime();

    if is_miller_rabin_prime(&prime_candidate, MILLER_RABIN_ITERATIONS) {
        return prime_candidate;
    }
    find_prime()
}
