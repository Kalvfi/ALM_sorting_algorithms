use std::time::Instant;

use alm_sorting_algorithms::read_lines;

fn main() {
    let mut items: Vec<i32> = read_lines("random_integers_10M.txt");

    let start = Instant::now();

    // Merge sort
    merge_sort(&mut items);

    println!("Finished in {:.2?}", start.elapsed());
}

fn merge_sort<T: Ord + Clone>(items: &mut Vec<T>) {
    let n = items.len();
    if n <= 1 {
        return;
    }

    let mid = n / 2;

    // Split the slice into two halves
    let mut left = items[..mid].to_vec();
    let mut right = items[mid..].to_vec();

    // Recursively sort the halves
    merge_sort(&mut left);
    merge_sort(&mut right);

    // Merge back into original slice
    merge(items, &left, &right);
}

fn merge<T: Ord + Clone>(items: &mut [T], left: &[T], right: &[T]) {
    let mut i = 0; // index for left
    let mut j = 0; // index for right
    let mut k = 0; // index for items

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            items[k] = left[i].clone();
            i += 1;
        } else {
            items[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        items[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        items[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
