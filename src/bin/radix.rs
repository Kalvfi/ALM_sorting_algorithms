use std::time::Instant;

use alm_sorting_algorithms::read_lines;

fn main() {
    let mut items: Vec<i32> = read_lines("random_integers_10M.txt");

    let start = Instant::now();

    // Radix sort
    radix_sort_i32(&mut items);

    println!("Finished in {:.2?}", start.elapsed());
}

fn radix_sort_i32(items: &mut Vec<i32>) {
    // Switch sign bit
    let mut tmp: Vec<u32> = items
        .iter()
        .map(|&x| (x as u32) ^ 0x8000_0000)
        .collect();

    // Sort
    radix_sort_u32(&mut tmp);

    // Switch back
    for (a, x) in items.iter_mut().zip(tmp) {
        *a = (x ^ 0x8000_0000) as i32;
    }
}

fn radix_sort_u32(items: &mut Vec<u32>) {
    let n = items.len();
    let mut output = vec![0; n];

    for shift in (0..32).step_by(8) {
        let mut count = [0; 256];

        // Counting
        for &x in items.iter() {
            let byte = ((x >> shift) & 0xff) as usize;
            count[byte] += 1;
        }

        // Prefix sums
        let mut sum = 0;
        for c in count.iter_mut() {
            let tmp = *c;
            *c = sum;
            sum += tmp;
        }

        // Build output
        for &x in items.iter() {
            let byte = ((x >> shift) & 0xff) as usize;
            output[count[byte]] = x;
            count[byte] += 1;
        }

        // Copy back
        items.copy_from_slice(&output);
    }
}

fn radix_sort_string(items: &mut Vec<String>) {
    let n = items.len();

    // Find maximum string length
    let max_len = items
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0);

    let mut output = vec![String::new(); n];

    for pos in (0..max_len).rev() {
        let mut count = [0; 257];

        // Counting
        for s in items.iter() {
            let byte = match s.as_bytes().get(pos) {
                Some(&b) => (b as usize) + 1,
                None => 0,
            };
            count[byte] += 1;
        }

        // Prefix sums
        let mut sum = 0;
        for c in count.iter_mut() {
            let tmp = *c;
            *c = sum;
            sum += tmp;
        }

        // Build output
        for s in items.iter() {
            let byte = match s.as_bytes().get(pos) {
                Some(&b) => (b as usize) + 1,
                None => 0,
            };
            output[count[byte]] = s.clone();
            count[byte] += 1;
        }

        // Copy back
        items.clone_from_slice(&output);
    }
}
