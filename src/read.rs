use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn readtovec() -> (Vec<f64>, Vec<f64>) {
    let mut btc: Vec<f64> = Vec::new();
    let mut eth: Vec<f64> = Vec::new();

    let file = File::open("data.txt");
    let reader = BufReader::new(file.unwrap());

    for line in reader.lines() {
        let x = line.unwrap();
        let parts: Vec<&str> = x.split(",").to_owned().collect();
        eth.push(parts[0].parse().unwrap());
        btc.push(parts[1].parse().unwrap());
    }
    return (eth.clone(), eth);
}
