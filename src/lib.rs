#![feature(test)]
extern crate test;

use std::collections::VecDeque;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
use slog::Drain;

/// As batches double from one to another, we need a starting point
/// MINIMAL_BATCH_SIZE allow us to tune this initial batch size.
/// Technically any number ≥ 1 is possible, but a huge batch size can reduce performance by
/// allocating too much continues memory.
///
/// The value is interpreted as a power of two, example MINIMAL_BATCH_SIZE=10 would mean an initial
/// batch size of 2^10 → 1024 numbers
const MINIMAL_BATCH_SIZE: u32 = 8;

pub struct Space {
    batches: VecDeque<Batch>,
    computed: Vec<Batch>,
    log: slog::Logger,
}

impl Space {
    pub fn new(power: u32, log: slog::Logger) -> Space {
        debug!(log, "Initialize");
        let mut batches = VecDeque::with_capacity(32);
        // Space allocation, can be threaded
        let mut starter = 0;
        for i in 0..batch_count(power) {
            debug!(log, "Allocationg {} size {}", i, MINIMAL_BATCH_SIZE + i);
            batches.push_back(Batch::new(
                MINIMAL_BATCH_SIZE + i,
                starter,
                log.new(o!(
                            "Start"=> starter,
                            "Size" => MINIMAL_BATCH_SIZE + i)),
            ));
            starter += 2usize.pow(MINIMAL_BATCH_SIZE + i);
        }
        Space {
            batches,
            computed: Vec::with_capacity(32),
            log,
        }
    }

    pub fn compute_all(&mut self) {
        debug!(self.log, "Compute …");
        while let Some(mut batch) = self.batches.pop_front() {
            batch.run(&self.computed);
            self.computed.push(batch);
        }
    }

    pub fn display_primes(&self, dots: bool) {
        let last = self.computed.last().unwrap();
        let longest = format!("{}", last.start + last.data.len() - 1);
        let longest = longest.len();
        if !dots {
            for batch in self.computed.iter() {
                println!(
                    "{:0size$}-{:0size$}: {:#}",
                    batch.start,
                    batch.start + batch.data.len() - 1,
                    batch,
                    size = longest
                );
            }
        } else {
            for batch in self.computed.iter() {
                println!(
                    "{:0size$}-{:0size$}: {:}",
                    batch.start,
                    batch.start + batch.data.len() - 1,
                    batch,
                    size = longest
                );
            }
        }
    }
}

struct Batch {
    data: Vec<bool>,
    start: usize,
    log: slog::Logger,
}

impl Batch {
    fn new(power: u32, starter: usize, log: slog::Logger) -> Batch {
        Batch {
            data: vec![false; 2usize.pow(power)],
            start: starter,
            log,
        }
    }

    fn sieve_prime(&mut self, prime: usize) {
        let mut from = self.start / prime;
        if from < 2 {
            from = 2;
        }
        let to = self.start + self.data.len();
        trace!(self.log, "Sieve: from = {}, prime = {}", from, prime);
        for i in from..to {
            let multiple = prime * i;
            if multiple >= to {
                break;
            }
            if multiple < self.start {
                continue;
            }
            // trace!(self.log, "Strike {}", multiple);
            self.data[multiple - self.start] = true;
        }
    }

    fn dot_stream(&self) -> impl Iterator<Item = char> + '_ {
        self.data.iter().map(|i| if *i { ' ' } else { '.' })
    }

    fn iter_primes(&self) -> impl Iterator<Item = usize> + '_ {
        let start = self.start;
        self.data
            .iter()
            .enumerate()
            .filter(|&(_, item)| !*item)
            .map(move |(i, _)| i + start)
    }

    fn run(&mut self, computed: &Vec<Batch>) {
        // 0 and 1 can't be marked automatically, mark them if there are included
        // then sieve on every number, not just prime from previous batches because there isn't
        if self.start <= 2 {
            if self.start == 0 {
                // mark 0 and perhaps 1 if included in this batch
                self.data[0] = true;
                if self.data.len() > 1 {
                    self.data[1] = true;
                }
            } else if self.start == 1 {
                // mark 1
                self.data[0] = true;
            } // else nothing to mark
            let to = self.data.len();
            for i in 2..to {
                if i.pow(2) > to {
                    break;
                }
                self.sieve_prime(i);
            }
        } else {
            let to = self.start + self.data.len();
            for batch in computed.iter() {
                for prime in batch.iter_primes() {
                    if prime.pow(2) > to {
                        break;
                    }
                    self.sieve_prime(prime);
                }
            }
        }
        debug!(self.log, "Completed: {:?}", self);
    }
}

impl std::fmt::Display for Batch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            for prime in self.iter_primes() {
                write!(f, "{} ", prime)?;
            }
        } else {
            for dot in self.dot_stream() {
                write!(f, "{}", dot)?;
            }
        }
        write!(f, "")
    }
}

impl std::fmt::Debug for Batch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut iter = self.iter_primes();
        if f.alternate() {
            write!(
                f,
                "Batch({}, {}) {{",
                self.start,
                self.start + self.data.len() - 1
            )?;
            if let Some(first_prime) = iter.next() {
                write!(f, "\n\t{}", first_prime)?;
            }
            for prime in iter {
                write!(f, ",\n\t{}", prime)?;
            }
            write!(f, "\n}}")
        } else {
            write!(
                f,
                "Batch({}, {}) {{",
                self.start,
                self.start + self.data.len() - 1
            )?;
            if let Some(first_prime) = iter.next() {
                write!(f, "{}", first_prime)?;
            }
            for prime in iter {
                write!(f, ", {}", prime)?;
            }
            write!(f, "}}")
        }
    }
}

/// Compute a number of batches to work on number of the maximum size of 2^power.
///
/// The first expected batch size is MINIMAL_BATCH_SIZE, the each batches double.
fn batch_count(power: u32) -> u32 {
    power.saturating_sub(MINIMAL_BATCH_SIZE) + 1
}

/// Generate a root logger
///
/// Log levels are between 0 and 2 which means Info, Debug, Trace
pub fn get_root_logger(digit_log_level: usize) -> slog::Logger {
    let lfilter = slog::Level::from_usize(digit_log_level + slog::Level::Info.as_usize()).unwrap();
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog::LevelFilter::new(drain, lfilter).map(slog::Fuse);
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_short_batch_count() {
        assert_eq!(batch_count(4), 1);
    }

    #[test]
    fn test_normal_batch_count() {
        assert_eq!(batch_count(10), 3);
        assert_eq!(batch_count(16), 9);
    }

    #[test]
    fn test_huge_batch_count() {
        // For values having more than usize size
        assert_eq!(batch_count(128), 121);
        assert_eq!(batch_count(4096), 4089);
    }

    #[bench]
    fn test_4_performance(b: &mut Bencher) {
        let log = get_root_logger(0);
        b.iter(move || {
            let log = log.clone();
            Space::new(4, log);
        });
    }

    #[bench]
    fn test_10_performance(b: &mut Bencher) {
        let log = get_root_logger(0);
        b.iter(move || {
            let log = log.clone();
            Space::new(10, log);
        });
    }

    #[bench]
    fn test_20_performance(b: &mut Bencher) {
        let log = get_root_logger(0);
        b.iter(move || {
            let log = log.clone();
            Space::new(20, log);
        });
    }
}
