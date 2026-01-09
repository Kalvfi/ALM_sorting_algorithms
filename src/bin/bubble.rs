use std::time::{ Instant, Duration };

use alm_sorting_algorithms::read_lines;

fn main() {
    let mut items: Vec<i32> = read_lines("random_integers_10M.txt");
    let n = items.len();

    let start = Instant::now();
    let one_hour = Duration::from_secs(3600);

    let mut new_n = n;

    for i in 0..n {
        let mut swapped = false;
        let mut last_swap = 0;

        // Timeout Check
        if i % 5000 == 0 {
            if start.elapsed() > one_hour {
                let percent = ((i as f64) / (n as f64)) * 100.0;
                println!("Sorted {:.2}%", percent);
                return;
            }
        }

        // Bubble Sort
        for j in 0..new_n - 1 {
            if items[j] > items[j + 1] {
                items.swap(j, j + 1);
                swapped = true;
                last_swap = j + 1;
            }
        }

        new_n = last_swap;

        if !swapped {
            break;
        }
    }

    println!("Finished in {:.2?}", start.elapsed());
}
