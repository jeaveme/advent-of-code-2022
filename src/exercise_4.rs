use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn former_contains_latter(range_a: &[usize], range_b: &[usize]) -> bool {
    range_a[0] <= range_b[0] && range_a[1] >= range_b[1]
}

fn some_overlap(range_a: &[usize], range_b: &[usize]) -> bool {
    !(range_a[1] < range_b[0] || range_a[0] > range_b[1])
}

pub fn camp_cleanup() -> Result<(), &'static str> {
    let file = File::open("resources/4_camp_cleanup.txt").map_err(|_| "Cannot open file.")?;
    let reader = BufReader::new(file);
    let mut count_fully_contained: usize = 0;
    let mut count_some_overlap: usize = 0;

    for line in reader.lines() {
        let ranges = line.map_err(|_| "Cannot read file.")?;
        let parsed_ranges = ranges
            .split(&[',', '-'])
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();
        let (range_a, range_b) = parsed_ranges.split_at(2);
        if former_contains_latter(range_a, range_b) || former_contains_latter(range_b, range_a) {
            count_fully_contained += 1;
        }
        if some_overlap(range_b, range_a) {
            count_some_overlap += 1;
        }
    }

    println!("Answer A: Ranges fully contained by the other = {count_fully_contained}");
    println!("Answer B: Ranges with overlap = {count_some_overlap}");

    Ok(())
}
