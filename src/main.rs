extern crate sieve;

fn main() {
    let mut space = sieve::Space::new(24);
    space.compute_all();
    space.display_primes();
}
