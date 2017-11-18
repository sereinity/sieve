#[macro_use]
extern crate clap;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

extern crate sieve;

use clap::{App, Arg, ErrorKind};
use slog::Drain;

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
    let lfilter = slog::Level::from_usize(digit_log_level + slog::Level::Info.as_usize()).unwrap();
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog::LevelFilter::new(drain, lfilter).map(slog::Fuse);
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!());

    let size = value_t!(matches, "size", u32).unwrap_or_else(|e|{
        match e.kind {
            ErrorKind::ArgumentNotFound => 16,
            _ => e.exit(),
        }
    });

    debug!(log, "Initialize");
    let mut space = sieve::Space::new(size);
    debug!(log, "Compute …");
    space.compute_all();
    debug!(log, "Done");
    if !matches.is_present("no-display") {
        space.display_primes();
    }
}
