use std::time::Instant;

use alm_sorting_algorithms::read_lines;

fn main() {
    let mut items: Vec<i32> = read_lines("random_integers_10M.txt");

    let start = Instant::now();

    // Quicksort
    quicksort(&mut items);

    println!("Finished in {:.2?}", start.elapsed());
}

fn quicksort<T: Ord>(items: &mut [T]) {
    let n = items.len();
    if n <= 1 {
        return;
    }

    // Split array into left and ride side of the pivot
    let pivot = partition(items);
    let (left, right) = items.split_at_mut(pivot);

    // Recursively sort left and right side
    quicksort(left);
    quicksort(&mut right[1..]);
}

fn partition<T: Ord>(items: &mut [T]) -> usize {
    let n = items.len();
    let pivot = n / 2;

    // Move pivot to the end
    items.swap(pivot, n - 1);

    let mut final_index = 0;

    for i in 0..n - 1 {
        if items[i] <= items[n - 1] {
            items.swap(i, final_index);
            final_index += 1;
        }
    }

    // Move pivot to its final place
    items.swap(final_index, n - 1);
    final_index
}
