use std::fs::File;

pub mod attempt_a;

pub trait Aggregator {
    fn aggregate(input: File);
}
