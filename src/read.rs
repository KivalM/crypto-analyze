use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn readtovec(filename: &str) -> (Vec<f64>, Vec<f64>) {
    let mut values: Vec<f64> = Vec::new();
    let mut mod_values: Vec<f64> = Vec::new();
    let file = File::open(filename);
    let reader = BufReader::new(file.unwrap());

    for line in reader.lines() {
        let x = line.unwrap();
        let parts: Vec<&str> = x.split(",").to_owned().collect();
        values.push(parts[0].parse().unwrap());
        mod_values.push(parts[1].parse().unwrap());
    }
    (values, mod_values)
}
