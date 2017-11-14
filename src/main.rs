extern crate sieve;

fn main() {
    let mut space = sieve::Space::new(16);
    for i in 2..16 {
        space.sieve_prime(i);
    }
    space.display_primes();
}
