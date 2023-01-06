use std::collections::BinaryHeap;
use std::fs;

use std::cmp::Reverse;

fn main() {
    let input_file = "data/day1.data";
    println!("Input at {}", input_file);

    let contents = fs::read_to_string(input_file).expect("Should be able to read file");

    let mut calorie_heap = BinaryHeap::with_capacity(contents.len());
    let (mut current, mut max) = (0, 0);

    for line in contents.split("\n") {
        if line == "" {
            // new elf so reset count
            //println!("Elf {} total: {}", n_elfs, current);
            calorie_heap.push(Reverse(current));
            if calorie_heap.len() > 3 {
                calorie_heap.pop();
            }
            current = 0;
            continue;
        }

        let calories = line.parse::<i32>().expect(&format!(
            "All lines should be numbers, but found '{}'",
            line
        ));

        current += calories;

        if current > max {
            max = current;
        }
    }
    println!("max: {}", max);
    println!(
        "heap max 3: {:?}",
        calorie_heap
            .iter()
            .map(|Reverse(x)| *x) // this is not super clean but 🤷
            .reduce(|accum, x| accum + x)
            .expect("there is at least one elf")
    );
}
