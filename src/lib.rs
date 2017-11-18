#![feature(conservative_impl_trait)]

pub struct Space {
    data: Vec<bool>,
}

impl Space {
    pub fn new(power: u32) -> Space {
        Space{
            data: vec!(false; 2usize.pow(power)),
        }
    }

    pub fn sieve_prime(&mut self, prime: usize) {
        let iterator = 2..self.data.len();
        let to = self.data.len();
        for i in iterator {
            let multiple = prime * i;
            if multiple >= to {
                break;
            }
            self.data[prime * i] = true;
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

    pub fn compute_all(&mut self) {
        for i in 2..self.data.len() {
            self.sieve_prime(i);
        }
    }
}
