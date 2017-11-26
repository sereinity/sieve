#![feature(conservative_impl_trait)]
#![feature(test)]
extern crate test;

use std::collections::VecDeque;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
use slog::Drain;


// Can change this parameter, the minimal minimal is 1, but it generates a bug on «4»
// const MINIMAL_BATCH_SIZE: u32 = 10;  // used as 2^MINIMAL_BATCH_SIZE
const MINIMAL_BATCH_SIZE: u32 = 2;  // used as 2^MINIMAL_BATCH_SIZE

pub struct Space {
    batches: VecDeque<Batch>,
    computed: Vec<Batch>,
}

impl Space {
    pub fn new(power: u32, log: &slog::Logger) -> Space {
        let mut batches = VecDeque::with_capacity(32);
        // Space allocation, can be threaded
        let mut starter = 0;
        for i in 0..batch_count(power) {
            debug!(log, "Allocationg {} size {}", i, MINIMAL_BATCH_SIZE + i);
            batches.push_back(Batch::new(MINIMAL_BATCH_SIZE + i, starter));
            starter += 2usize.pow(MINIMAL_BATCH_SIZE + i);
        }
        Space {
            batches,
            computed: Vec::with_capacity(32),
        }
    }

    pub fn compute_all(&mut self) {
        while let Some(mut batch) = self.batches.pop_front() {
            batch.run(&self.computed);
            self.computed.push(batch);
        }
    }

    pub fn display_primes(&self) {
        for batch in self.computed.iter() {
            batch.display_primes();
        }
    }
}

struct Batch {
    data: Vec<bool>,
    start: usize,
}

impl Batch {
    fn new(power: u32, starter: usize) -> Batch {
        Batch {
            data: vec!(false; 2usize.pow(power)),
            start: starter,
        }
    }

    fn sieve_prime(&mut self, prime: usize) {
        let to = self.start + self.data.len();
        // println!("Sieve: start = {}, prime = {}", self.start, prime);
        for i in 2..to {
            let multiple = prime * i;
            if multiple >= to {
                break;
            }
            if multiple < self.start {
                continue;
            }
            // println!("multiple: {}", multiple);
            self.data[multiple - self.start] = true;
        }
    }

    fn display_primes(&self) {
        print!("{:08}: ", self.start);
        for dot in self.dot_stream() {
            print!("{}", dot);
        }
        println!("");
    }

    fn dot_stream<'a>(&'a self) -> impl Iterator<Item=char> + 'a {
        self.data.iter()
            .map(|i| {
                if *i {
                    ' '
                } else {
                    '.'
                }
            })
    }

    fn iter_primes<'a>(&'a self) -> impl Iterator<Item=usize> + 'a {
        let start = self.start;
        self.data.iter().enumerate().filter(|&(_, item)| {!*item}).map(move |(i, _)| {i + start})
    }

    fn run(&mut self, computed: &Vec<Batch>) {
        if self.start == 0 {  // incompatible with MINIMAL_BATCH_SIZE = 1
            self.data[0] = true;
            self.data[1] = true;
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
    }
}

/// Compute a number of batches to work on number of the maximum size of 2^power.
///
/// The first expected batch size is MINIMAL_BATCH_SIZE, the each batches double.
fn batch_count(power: u32) -> u32 {
    power.saturating_sub(MINIMAL_BATCH_SIZE - 1) + 1
}

/// Generate a root logger
pub fn get_root_logger(digit_log_level: usize) -> slog::Logger{
    let lfilter = slog::Level::from_usize(digit_log_level + slog::Level::Info.as_usize()).unwrap();
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
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
        assert_eq!(batch_count(10), 2);
        assert_eq!(batch_count(16), 8);
    }

    #[test]
    fn test_huge_batch_count() {
        // For values having more than usize size
        assert_eq!(batch_count(128), 120);
        assert_eq!(batch_count(4096), 4088);
    }

    #[bench]
    fn test_20_performance(b: &mut Bencher) {
        let log = get_root_logger(0);
        b.iter(move || {
            Space::new(20, &log);
        });
    }
}
