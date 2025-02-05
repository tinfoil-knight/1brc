use std::fs::File;

use one_brc::{attempt_a::AttemptA, attempt_b::AttemptB, Aggregator};

fn main() {
    let path = "data/measurements.txt";
    let mut file = File::open(path).unwrap();
    AttemptB::aggregate(&mut file);
}
