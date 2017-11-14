extern crate sieve;

fn main() {
    let mut space = sieve::Space::new(1024);
    space.compute_all();
    space.display_primes();
}
