use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_aggregates(input: File) {
    let buffered = BufReader::new(input);

    // count, sum, min, max
    let mut map: BTreeMap<String, [f64; 4]> = BTreeMap::new();

    for line in buffered.lines() {
        let ln = line.unwrap();
        let (station_name, measurement_s) = ln.split_once(';').unwrap();
        let measurement = measurement_s.parse::<f64>().unwrap();

        match map.get_mut(station_name) {
            Some(x) => {
                *x = [
                    x[0] + 1_f64,
                    x[1] + measurement,
                    if measurement < x[2] {
                        measurement
                    } else {
                        x[2]
                    },
                    if measurement > x[3] {
                        measurement
                    } else {
                        x[3]
                    },
                ];
            }
            None => {
                map.insert(
                    station_name.to_owned(),
                    [1_f64, measurement, measurement, measurement],
                );
            }
        }
    }

    for entry in map {
        let (station_name, [count, sum, min, max]) = entry;
        let mean = ((sum / count) * 10.0).round() / 10.0;
        println!("{station_name}={min}/{mean}/{max}");
    }
}

fn main() {
    let path = "data/measurements.txt";
    calculate_aggregates(File::open(path).unwrap());
}
