use rand::{rngs::SmallRng, RngCore, SeedableRng};
use std::time::Instant;
use std::fs;
use std::path::Path;
use std::io::Write;
use std::thread;

mod hashmap;
mod prime;
use hashmap::AdvancedHashMap;
use prime::find_primes;

fn main() {

    let t1 = thread::spawn(|| test(1, 0.5, 5_000, 500));
    let t2 = thread::spawn(|| test(2, 0.9, 5_000, 500));
    let t3 = thread::spawn(|| test(3, 0.95, 5_000, 500));
    let t4 = thread::spawn(|| test(4, 1.0, 5_000, 500));

    t1.join();
    t2.join();
    t3.join();
    t4.join();

    // test(1, 0.5, 5_000, 500);
    // test(2, 0.9, 5_000, 500);
    // test(3, 0.95, 5_000, 500);
    // test(4, 1.0, 5_000, 500);
    

    // test_hashmap_collsion(10903, 0.5, 10_000, 500);
    // test_hashmap_collsion(10903, 0.9, 10_000, 500);
    // test_hashmap_collsion(10903, 0.95, 10_000, 500);
    // test_hashmap_collsion(10903, 1.0, 10_000, 500);
    
    
}

fn test(test_num: u8, percentage:f32, runs: u32, test_cases: u32) {
    let time: Instant = Instant::now();
    let primes: Vec<u32> = find_primes(100, 10_000);
    println!("Primes fetching took {}ms", time.elapsed().as_millis());
    
    let mut all_data: Vec<TestData> = Vec::new();
    println!("Starting test no. {}", test_num);
    let tmp: u32 = primes.len() as u32 / 100;
    for i in 0..primes.len() {
        if i as u32 % tmp == 0 {
            println!("{}: {}% complete...", test_num, i as u32 / tmp);
        }
        let test_data: TestData = test_hashmap_collsion(primes[i], percentage, runs, test_cases);
        all_data.push(test_data);
        // Debug
        // println!("{:?}", test_data);
    }
    let filename: &str = &format!("test{}", test_num);
    save_to_file(filename, all_data);
}


fn save_to_file(filename: &str, data: Vec<TestData>)-> std::io::Result<()>{
    let dir: &str = "./data/";
    let filepath: String = format!("{}{}", dir, filename);
    if !Path::new(dir).exists() {
        fs::create_dir(dir)?;
    }

    let mut file: fs::File;
    if Path::new(&filepath).exists() {
        file = fs::File::create(filepath).unwrap();
    }
    else {
        file = fs::OpenOptions::new()
            .read(true)
            .truncate(true)
            .write(true)
            .create(true)
            .open(filepath)
            .unwrap();
    }
    
    for d in data {
        let line = format!("{},{},{},{},{},{},{},{}", d.hashmap_size, d.percentage, d.runs, d.test_cases, 
        d.test_time, d.collision, d.avg_collsion, d.collision_rate);
        writeln!(file, "{}", line);
    }
    println!("Saving to file successful...");
    Ok(())
}

#[derive(Debug, Copy, Clone)]
struct TestData {
    hashmap_size: u32,
    percentage: f32,
    runs: u32,
    test_cases: u32,
    test_time: f32,
    collision: u64,
    avg_collsion: f64,
    collision_rate: f64
}

fn test_hashmap_collsion(hashmap_size: u32, percentage: f32, runs: u32, test_cases: u32) -> TestData {
    let mut hashmap: AdvancedHashMap<u32>;
    let mut rng: SmallRng = SmallRng::from_entropy();
    let time: Instant = Instant::now();
    let mut collision: u64 = 0;

    for _ in 0..runs {
        hashmap = hashmap::get_filled_hashmap(hashmap_size, percentage);
        for _ in 0..test_cases {
            if hashmap.insert(rng.next_u32(), rng.next_u32()) == true {
                collision += 1;
            }
        }

    }
    let test_time = time.elapsed().as_nanos() as f32 / 1_000_000.0;
    let avg_collsion: f64 = collision as f64 / runs as f64;
    let collision_rate: f64 = (avg_collsion / test_cases as f64) * 100.0;

    // println!("Finished test in {}ms...", test_time);
    // println!("{0} Collsions happend in {1} runs with {2} tests...", collision, runs, test_cases);
    // println!("Hashmap size: {} with {}% filled", hashmap_size, percentage * 100.0);
    // println!("Average collsion per run: {}", avg_collsion);
    // println!("Collision rate: {}%", collision_rate);
    // println!("-----------------------------------------------------------------------------------------------------------------");
    
    TestData {
        hashmap_size: hashmap_size,
        percentage: percentage,
        runs: runs,
        test_cases: test_cases,
        test_time: test_time,
        collision: collision,
        avg_collsion: avg_collsion,
        collision_rate: collision_rate
    }

}
