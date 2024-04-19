use rsa_algorithm::prime_num_gen;

const RSA_SIZE: u32 = 4096;
const PRIME_SIZE: u32 = RSA_SIZE / 2;
fn main() {
    let p = prime_num_gen::find_prime(PRIME_SIZE);
    // let q = prime_num_gen::find_prime();
    // println!("p: {p}\nq: {q}");
    println!("p: {p}");
}
