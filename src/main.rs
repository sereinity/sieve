extern crate sieve;

fn main() {
    let max :usize = 1024;
    let mut space = sieve::Space::new(max);
    for i in 2..max {
        space.sieve_prime(i);
    }
    space.display_primes();
}
