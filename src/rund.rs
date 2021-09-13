use std::{thread, time::Duration};

pub fn simulate(pd: f64, td: usize, pu: f64, tu: usize, det: Vec<f64>, curr: Vec<f64>) -> f64 {
    let mut ether = 0.0;
    let mut usdt = 100.0;

    for i in 0..det.len() {
        if (i < tu) || (i < td) {
            continue;
        }
        let buy: bool;
        let sell: bool;

        let new = det[i];
        let old = det[i - tu];

        let percent = ((new - old) / old) * 100.0;
        buy = percent >= pu as f64;

        let new = det[i];
        let old = det[i - td];

        let percent = ((new - old) / old) * 100.0;

        sell = percent <= (pd) as f64;

        if buy && sell {
            // nothing
        } else if buy {
            if usdt != 0.0 {
                ether = usdt / curr[i];
                usdt = 0.0;
                ether = ether * (99.9 / 100.0);
            }
        } else if sell {
            if ether != 0.0 {
                usdt = ether * curr[i];
                ether = 0.0;
                usdt = usdt * (99.9 / 100.0);
            }
        }
    }
    let val: f64 = usdt + (ether * curr[curr.len() - 1]);
    return val;
}
pub fn optim_sell(
    perc_up: f64,
    time_up: usize,
    d1: i32,
    d2: i32,
    dt: usize,
    det: Vec<f64>,
    curr: Vec<f64>,
) {
    let mut maxval = 0.0;

    for pd in d1..d2 {
        for time_down in 1..dt {
            thread::sleep(Duration::from_millis(10));
            let perc_d: f64 = (pd as f64) / 1000.0;
            let val = simulate(
                perc_d,
                time_down,
                perc_up,
                time_up,
                det.clone(),
                curr.clone(),
            );
            if val > maxval {
                maxval = val;
                println!(
                    "VAL: {} time_d: {} perc_d: {} time_u: {} perc_u {}",
                    val, time_down, perc_d, time_up, perc_up
                )
            }
        }
    }
}

pub fn optim_buy(
    perc_d: f64,
    time_d: usize,
    u1: i32,
    u2: i32,
    ut: usize,
    det: Vec<f64>,
    curr: Vec<f64>,
) {
    let mut maxval = 0.0;

    for pu in u1..u2 {
        for time_up in 1..ut {
            thread::sleep(Duration::from_millis(10));
            let perc_u: f64 = (pu as f64) / 1000.0;
            let val = simulate(perc_d, time_d, perc_u, time_up, det.clone(), curr.clone());
            if val > maxval {
                maxval = val;
                println!(
                    "VAL: {} time_d: {} perc_d: {} time_u: {} perc_u {}",
                    val, time_d, perc_d, time_up, perc_u
                )
            }
        }
    }
}
