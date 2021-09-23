use std::{
    io,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    gen::{exists, generate},
    graph::graphit,
    model::Values,
    parallel::{t_buy, t_sell},
    read::readtovec,
};

pub fn get_input(message: &str) -> String {
    println!("{}", message);
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    buffer = buffer
        .to_uppercase()
        .strip_suffix("\n")
        .unwrap()
        .to_string();
    buffer
}

pub fn interactive() {
    let b = get_input("Enter the symbol of the pair of your choice ETHUSDT/ADAUSDT etc.");
    let pair = b.clone();

    println!("You have entered: {}", b);

    if exists(format!("{}.txt", &pair)) {
        let y = get_input("File exists, do you want to update the info in the file");
        if y == "Y" {
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            let in_ms = since_the_epoch.as_secs() * 1000
                + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
            generate(&pair, 1609452000000, in_ms);
            println!("Generated {}.txt", &pair);
        }
    } else {
        println!("generating {}.txt", &pair);
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let in_ms =
            since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
        generate(&pair, 1609452000000, in_ms);
        println!("Generated {}.txt", &pair);
    }
    let filename = pair + ".txt";
    let (p, m) = readtovec(&filename);
    let mut t = Values {
        perc_up: 0.209,
        perc_down: -1.999,
        time_down: 34,
        time_up: 151,
        profit: 0.0,
        prices: p.clone(),
        modified: m.clone(),
    };

    let mut b = get_input("Start Running Simulations?");
    println!("{}", b);
    //0.209 151 -1.999 34
    while b == "Y" {
        t = t_buy(0.0, 3.0, 288, t);
        println!(
            "profit% {} perc up {} time up{} perc down{} time down{}",
            t.profit, t.perc_up, t.time_up, t.perc_down, t.time_down
        );
        t = t_sell(-3.0, -1.0, 288, t);
        println!(
            "profit% {} perc up {} time up{} perc down{} time down{}",
            t.profit, t.perc_up, t.time_up, t.perc_down, t.time_down
        );

        b = get_input("Run again?");
    }
    graphit(&t);
}
