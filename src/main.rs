#[macro_use]
extern crate clap;
#[macro_use]
extern crate slog;

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
        .arg(Arg::with_name("no-display")
             .long("--no-display")
             .short("-q")
             .help("Don't display the result")
             )
        .arg(Arg::with_name("dots")
             .long("--dots")
             .help("Display dots instead of values")
             )
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity")
             )
        .get_matches();

    // First "v" adds debug, the second trace, trace is not available directly
    // As there isn't more log levels, we limits to 2 level of more verbosity
    let mut digit_log_level = matches.occurrences_of("v") as usize;
    if digit_log_level > 2 {
        digit_log_level = 2;
    }
    let log = sieve::get_root_logger(digit_log_level);

    let size = value_t!(matches, "size", u32).unwrap_or_else(|e|{
        match e.kind {
            ErrorKind::ArgumentNotFound => 16,
            _ => e.exit(),
        }
    });

    let mut space = sieve::Space::new(size, log.new(o!("Space" => 1)));
    space.compute_all();
    if !matches.is_present("no-display") {
        space.display_primes(matches.is_present("dots"));
    }
}
