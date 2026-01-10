use std::time::Instant;

use alm_sorting_algorithms::read_lines;

fn main() {
    let mut items: Vec<i32> = read_lines("random_integers_10M.txt");

    let start = Instant::now();

    // Heapsort
    heapsort(&mut items);

    println!("Finished in {:.2?}", start.elapsed());
}

fn heapsort<T: Ord>(items: &mut Vec<T>) {
    let n = items.len();

    // Build max heap
    for start in (0..n / 2).rev() {
        sift_down(items, start, n);
    }

    // Extract elements one by one
    for end in (1..n).rev() {
        items.swap(0, end);
        sift_down(items, 0, end);
    }
}

fn sift_down<T: Ord>(items: &mut Vec<T>, mut root: usize, end: usize) {
    loop {
        let left = 2 * root + 1;
        if left >= end {
            break;
        }

        let mut child = left;
        let right = left + 1;

        if right < end && items[right] > items[left] {
            child = right;
        }

        if items[child] > items[root] {
            items.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}
