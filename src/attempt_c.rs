use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Aggregator;

pub struct AttemptC;

impl Aggregator for AttemptC {
    fn aggregate(input: &mut File) {
        let buffered = BufReader::new(input);

        // count, sum, min, max
        let mut map: HashMap<String, [f64; 4]> = HashMap::new();

        buffered.lines().for_each(|line| {
            let ln = line.unwrap();

            let (station_name, measurement) = ln.split_once(';').unwrap();
            let measurement = measurement.parse::<f64>().unwrap();

            map.entry(station_name.to_owned())
                .and_modify(|x| {
                    *x = [
                        x[0] + 1_f64,
                        x[1] + measurement,
                        x[2].min(measurement),
                        x[3].max(measurement),
                    ]
                })
                .or_insert([1_f64, measurement, measurement, measurement]);
        });

        let mut station_names: Vec<&String> = map.keys().collect();
        station_names.sort_unstable();

        for station_name in station_names {
            let [count, sum, min, max] = map.get(station_name).unwrap();
            let mean = ((sum / count) * 10.0).round() / 10.0;
            println!("{station_name}={min}/{mean}/{max}");
        }
    }
}
