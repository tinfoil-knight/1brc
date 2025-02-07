use std::fs::File;

use one_brc::{attempt_c::AttemptC, Aggregator};

fn main() {
    let path = "data/measurements.txt";
    let mut file = File::open(path).unwrap();
    AttemptC::aggregate(&mut file);
}
