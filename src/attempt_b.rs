use std::{
    collections::BTreeMap,
    f64::{INFINITY, NEG_INFINITY},
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Aggregator;

pub struct AttemptB;

impl Aggregator for AttemptB {
    fn aggregate(input: &mut File) {
        let buffered = BufReader::new(input);
        let mut map: BTreeMap<String, Vec<f64>> = BTreeMap::new();

        for line in buffered.lines() {
            let ln = line.unwrap();
            let (station_name, measurement_s) = ln.split_once(';').unwrap();
            let measurement = measurement_s.parse::<f64>().unwrap();

            map.entry(station_name.to_owned())
                .and_modify(|e| e.push(measurement))
                .or_insert(vec![measurement]);
        }

        for entry in map {
            let (station_name, measurements) = entry;

            let count = measurements.len() as f64;

            let (mut min_m, mut max_m, mut sum) = (INFINITY, NEG_INFINITY, 0.0);
            for m in measurements {
                min_m = min_m.min(m);
                max_m = max_m.max(m);
                sum += m;
            }

            let mean = ((sum / count) * 10.0).round() / 10.0;
            println!("{station_name}={min_m}/{mean}/{max_m}");
        }
    }
}
