use std::fs::File;

pub mod attempt_a;
pub mod attempt_b;
pub mod attempt_c;

pub trait Aggregator {
    fn aggregate(input: &mut File);
}
