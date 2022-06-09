pub fn find_primes(start: u32, up_to: u32) -> Vec<u32> {
    let mut primes: Vec<bool> = Vec::new();
    let mut real_primes: Vec<u32> = Vec::new();

    println!("Allocating vector");
    for _ in 0..up_to {
        primes.push(true);
    }
    println!("Finished allocating vector");

    println!("{}", (up_to as f32).sqrt() as usize);
    for i in 2..(up_to as f32).sqrt() as usize {
        if primes[i] == true {
            let mut j: u32 = i as u32 * i as u32;
            while j < up_to {
                primes[j as usize] = false;
                j = j + i as u32;
            }
        }
        
    }

    for i in 0..up_to {
        if primes[i as usize] == true && i >= start {
            real_primes.push(i as u32);
        }
    }

    real_primes
}