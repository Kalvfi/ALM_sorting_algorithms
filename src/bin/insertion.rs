use std::time::{ Instant, Duration };

use alm_sorting_algorithms::read_lines;

fn main() {
    let mut items: Vec<i32> = read_lines("random_integers_10M.txt");
    let n = items.len();

    let start = Instant::now();
    let one_hour = Duration::from_secs(3600);

    for i in 1..n {
        // Timeout Check
        if i % 5000 == 0 {
            if start.elapsed() > one_hour {
                let percent = ((i as f64) / (n as f64)) * 100.0;
                println!("Sorted {:.2}%", percent);
                return;
            }
        }

        // Insertion Sort
        let key = items[i];
        let mut j = i;
        while j > 0 && items[j - 1] > key {
            items[j] = items[j - 1];
            j -= 1;
        }
        items[j] = key;
    }

    println!("Finished in {:.2?}", start.elapsed());
}
