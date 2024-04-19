mod prime_num_gen;

use num_bigint::{BigUint, RandBigInt};

const RSA_SIZE: u32 = 4096;
const PRIME_SIZE: u32 = RSA_SIZE / 2;
const MILLER_RABIN_ITERATIONS: u32 = 64;

fn main() {
    let p = prime_num_gen::find_prime();
    // let q = prime_num_gen::find_prime();
    // println!("p: {p}\nq: {q}");
    println!("p: {p}");
}
