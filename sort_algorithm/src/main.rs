use rand::{RngCore, SeedableRng, rngs::SmallRng};
use std::time::Instant;

fn main() {
    let list_size: u32 = 100_000;

    let mut i: u32 = 10;
    let instant: Instant = Instant::now();
    loop {
        if i > list_size {
            break;
        }

        test_insert_sort(i as u32);
        test_bubble_sort(i as u32);
        i *= 10;
    }
    println!("+===============================================================================+");
    println!("{}ms elapsed", instant.elapsed().as_nanos() as f64 / 1_000_000.0);
}

fn test_insert_sort(list_size: u32){
    println!("+===============================================================================+");
    println!("| [Insert Sort (List size: {:9})]                                          |", list_size);
    
    let l1 = generate_random_list(list_size);
    let x1 = insert_sort(l1);
    println!("| [Random]: Comparisons: {0:11} | Swaps: {1:11} | Time: {2:11}ms |", x1.1, x1.2, x1.3);
    
    let l2 = generate_sorted_list(list_size);
    let x2 = insert_sort(l2);
    println!("| [Sorted]: Comparisons: {0:11} | Swaps: {1:11} | Time: {2:11}ms |", x2.1, x2.2, x2.3);
    
    let l3 = generate_invers_sorted_list(list_size);
    let x3 = insert_sort(l3);
    println!("| [Invers]: Comparisons: {0:11} | Swaps: {1:11} | Time: {2:11}ms |", x3.1, x3.2, x3.3);
}

fn test_bubble_sort(list_size: u32) {
    println!("+===============================================================================+");
    println!("| [Bubble Sort (List size: {:9})]                                          |", list_size);
    
    let l1 = generate_random_list(list_size);
    let x1 = bubble_sort(l1);
    println!("| [Random]: Comparisons: {0:11} | Swaps: {1:11} | Time: {2:11}ms |", x1.1, x1.2, x1.3);
    
    let l2 = generate_sorted_list(list_size);
    let x2 = bubble_sort(l2);
    println!("| [Sorted]: Comparisons: {0:11} | Swaps: {1:11} | Time: {2:11}ms |", x2.1, x2.2, x2.3);
    
    let l3 = generate_invers_sorted_list(list_size);
    let x3 = bubble_sort(l3);
    println!("| [Invers]: Comparisons: {0:11} | Swaps: {1:11} | Time: {2:11}ms |", x3.1, x3.2, x3.3);
}

fn insert_sort(list: Vec<u32>) -> (Vec<u32>, u64, u64, f64) {
    let mut sorted_list:Vec<u32> = list.clone();
    let mut comparisons:u64 = 0;
    let mut swaps:u64 = 0;
    let instant: Instant = Instant::now();
    

    for i in 0..list.len() {
        for j in 0..i {
            comparisons += 1;
            if sorted_list[i] < sorted_list[j] {
                let temp: u32 = sorted_list[j];
                sorted_list[j] = sorted_list[i];
                sorted_list[i] = temp;
                swaps += 1;
            }
        }
    } 

    let duration_ms: f64 = instant.elapsed().as_nanos() as f64 / 1_000_000.0;

    (sorted_list, comparisons, swaps, duration_ms)
}

fn is_sorted(list: &Vec<u32>) -> (bool, u64) {
    let mut comp: u64 = 0;
    for i in 0..list.len()-1 {
        comp += 1;
        if list[i] > list[i+1] {
            return (false, comp);
        }
    }
    (true, comp)
}

fn bubble_sort(list: Vec<u32>) -> (Vec<u32>, u64, u64, f64) {
    let mut sorted_list:Vec<u32> = list.clone();
    let mut comparisons:u64 = 0;
    let mut swaps:u64 = 0;
    let instant: Instant = Instant::now();

    loop {
        let sorted = is_sorted(&sorted_list);
        comparisons += sorted.1;        
        if sorted.0 {
            break;
        }

        for i in 0..list.len()-1 {
            comparisons += 1;
            if sorted_list[i] > sorted_list[i+1] {
                let temp: u32 = sorted_list[i+1];
                sorted_list[i+1] = sorted_list[i];
                sorted_list[i] = temp;
                swaps += 1;
            }
        }
    }

    let duration_ms: f64 = instant.elapsed().as_nanos() as f64 / 1_000_000.0;

    (sorted_list, comparisons, swaps, duration_ms)
}

fn generate_sorted_list(size: u32) -> Vec<u32> {
    let mut list: Vec<u32> = Vec::new();
    for i in 0..size {
        list.push(i as u32);
    }
    list
}

fn generate_invers_sorted_list(size: u32) -> Vec<u32> {
    let mut list: Vec<u32> = Vec::new();
    for i in 0..size {
        list.push(size - 1 - i);
    }
    list
}

fn generate_random_list(size: u32) -> Vec<u32> {
    let mut rng: SmallRng = SmallRng::from_entropy();
    let mut l: Vec<u32> = vec![0; size as usize];
    for i in 0..size {
        l[i as usize] = rng.next_u32() % 1_000;
    }
    l
}

