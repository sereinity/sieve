#[macro_use]
extern crate clap;
extern crate sieve;

use clap::{App, Arg, ErrorKind};

fn main() {
    let matches = App::new("sieve")
        .about("Optimized Erathostenese's sieve")
        .author("Sereinity <sereinity@online.fr>")
        .version(crate_version!())
        .arg(Arg::with_name("size")
             .help("Analyze space size in power of two, ie 10, 24")
             .index(1)
            )
        .get_matches();

    let size = value_t!(matches, "size", u32).unwrap_or_else(|e|{
        match e.kind {
            ErrorKind::ArgumentNotFound => 16,
            _ => e.exit(),
        }
    });

    let mut space = sieve::Space::new(size);
    space.compute_all();
    space.display_primes();
}
