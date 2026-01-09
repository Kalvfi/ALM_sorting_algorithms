use std::time::{ Instant, Duration };

use alm_sorting_algorithms::read_lines;

fn main() {
    let mut items: Vec<i32> = read_lines("random_integers_10M.txt");
    let n = items.len();

    let start = Instant::now();
    let one_hour = Duration::from_secs(3600);

    for i in 0..n {
        // Timeout Check
        if i % 5000 == 0 {
            if start.elapsed() > one_hour {
                let percent = ((i as f64) / (n as f64)) * 100.0;
                println!("Sorted {:.2}%", percent);
                return;
            }
        }

        // Selection Sort
        let mut min_index = i;
        for j in i + 1..n {
            if items[j] < items[min_index] {
                min_index = j;
            }
        }

        if min_index != i {
            items.swap(i, min_index);
        }
    }

    println!("Finished in {:.2?}", start.elapsed());
}
