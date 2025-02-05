use std::fs::File;

use one_brc::{attempt1::Attempt1, Aggregator};

fn main() {
    let path = "data/measurements.txt";
    Attempt1::aggregate(File::open(path).unwrap());
}
