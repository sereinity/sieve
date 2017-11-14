#![feature(conservative_impl_trait)]

pub struct Space {
    data: Vec<bool>,
}

impl Space {
    pub fn new(size: usize) -> Space {
        Space{
            data: vec!(false; size),
        }
    }

    pub fn sieve_prime(&mut self, prime: usize) {
        let from = 2;
        let to = 16;
        let mut i = from;
        loop {
            let multiple = prime * i;
            if multiple >= to {
                break;
            }

            self.data[multiple] = true;
            i += 1;
        }
    }

    pub fn display_primes(&self) {
        print!("Primes: ");
        for prime in self.iter_primes() {
            print!("{} ", prime);
        }
        println!("");
    }

    fn iter_primes<'a>(&'a self) -> impl Iterator<Item=usize> + 'a {
        self.data.iter().enumerate().filter(|&(_, item)| {!*item}).map(|(i, _)| {i})
    }
}
