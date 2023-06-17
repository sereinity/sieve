#[macro_use]
extern crate slog;

use clap::{value_parser, ArgAction, Parser};

#[derive(Parser)]
#[command(version)]
#[command(author)]
#[command(about = "Optimized Erathostenese's sieve")]
struct Cli {
    #[arg(help = "Analyze space size in power of two, ie 10, 24")]
    #[arg(value_parser = value_parser!(u32))]
    #[arg(default_value = "16")]
    size: u32,

    #[arg(short = 'q', long)]
    #[arg(help = "Don't display the result")]
    no_display: bool,

    #[arg(long)]
    #[arg(help = "Display dots instead of values")]
    dots: bool,

    #[arg(short, action = ArgAction::Count)]
    #[arg(help = "Sets the level of verbosity")]
    verbose: u8,
}

fn main() {
    let cli = Cli::parse();

    // First "v" adds debug, the second trace, trace is not available directly
    // As there isn't more log levels, we limits to 2 level of more verbosity
    let log = sieve::get_root_logger(cli.verbose.min(2) as usize);

    let mut space = sieve::Space::new(cli.size, log.new(o!("Space" => 1)));
    space.compute_all();

    if !cli.no_display {
        space.display_primes(cli.dots);
    }
}
