use std::time::{Duration, Instant};

fn main() {
    const NTH_PRIME: usize = 100_000_001;
    const BIG_PRIME: u64 = 2_305_843_009_213_693_951;

    let sqrtcheck_start: Instant = Instant::now();
    let sqrt_of_big_prime: f64 = (BIG_PRIME as f64).sqrt();
    let sqrtcheck_duration: Duration = sqrtcheck_start.elapsed();
    
    println!("Time taken for sqrt is {:?}", sqrtcheck_duration);
    println!("Sqrt of {} is {}", BIG_PRIME, sqrt_of_big_prime);
    
    let primecheck_start: Instant = Instant::now();
    println!("{} test for prime is {}", BIG_PRIME, primal::is_prime(BIG_PRIME));
    let primecheck_duration: Duration = primecheck_start.elapsed();

    println!("Time taken for check is {:?}", primecheck_duration);
    
    //let first_start = Instant::now();
    //let first_p = primal::Primes::all().nth(NTH_PRIME - 1).unwrap();
    //let first_duration = first_start.elapsed();

    let second_start: Instant = Instant::now();
    let second_p: usize = primal::StreamingSieve::nth_prime(NTH_PRIME);
    let second_duration: Duration = second_start.elapsed();

    //println!("The {}st prime is {}", NTH_PRIME, first_p);
    //println!("Time taken is {:?}", first_duration);

    println!("The {}st prime is {}", NTH_PRIME, second_p);
    println!("Time taken is {:?}", second_duration);
}
