use std::fs;

fn main() {
    let input_file = "data/day1.data";
    println!("Input at {}", input_file);

    let contents = fs::read_to_string(input_file).expect("Should be able to read file");

    let (mut current, mut max, mut n_elfs) = (0, 0, 0);
    for line in contents.split("\n") {
        if line == "" {
            // new elf so reset count
            println!("Elf {} total: {}", n_elfs, current);
            current = 0;
            n_elfs += 1;
            continue;
        }

        let calories = line
            .parse::<i32>()
            .expect(&format!("All lines should be numbers, but found '{}'", line));

        current += calories;

        if current > max {
            max = current;
        }
    }
    println!("max: {}", max);
}
