fn main() {
    let mut space = vec![false; 16];
    for i in 2..16 {
        sieve(&mut space, i, 2, 16);
    }
    display_primes(&space);
}

fn sieve(space: &mut Vec<bool>, value: usize, from: usize, to: usize) {
    let mut i = from;
    loop {
        let multiple = value * i;
        if multiple >= to {
            break;
        }

        space[multiple] = true;
        i += 1;
    }
}

fn display_primes(space: &Vec<bool>) {
    print!("Primes: ");
    for i in 0..space.len() {
        if !space[i] {
            print!("{} ", i);
        }
    }
    println!("");
}
