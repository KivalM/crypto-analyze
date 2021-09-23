use std::convert::TryInto;

use crate::{buy::optimum_buy, model::Values, sell::optimum_sell};

pub fn t_buy(r1: f64, r2: f64, time_range: usize, token: Values) -> Values {
    let threads = 12;
    let range = r2 - r1;
    let inc = range / threads as f64;

    let mut nums = Vec::new();

    for i in 0..threads {
        let d1 = r1 + (i as f64 * inc);

        nums.push(d1);
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads.try_into().unwrap())
        .build()
        .unwrap();

    let (tx, rx) = std::sync::mpsc::channel();

    for x in nums {
        let z = token.clone();
        let tx = tx.clone();
        pool.spawn(move || {
            tx.send(optimum_buy(z, (x).into(), (x + inc).into(), time_range, 3))
                .unwrap();
        });
    }
    drop(tx);
    let mut maxval = Values {
        perc_up: 1.0,
        perc_down: 1.0,
        time_down: 3,
        time_up: 3,
        profit: 0.0,
        prices: Vec::new(),
        modified: Vec::new(),
    };
    let val: Vec<Values> = rx.into_iter().collect();
    for z in val {
        if z.profit > maxval.profit {
            maxval = z.clone();
        }
    }
    maxval
}

pub fn t_sell(r1: f64, r2: f64, time_range: usize, token: Values) -> Values {
    let threads = 12;
    let range = r2 - r1;
    let inc = range / threads as f64;

    let mut nums = Vec::new();

    for i in 0..threads {
        let d1 = r1 + (i as f64 * inc);

        nums.push(d1);
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads.try_into().unwrap())
        .build()
        .unwrap();

    let (tx, rx) = std::sync::mpsc::channel();

    for x in nums {
        let z = token.clone();
        let tx = tx.clone();
        pool.spawn(move || {
            tx.send(optimum_sell(z, (x).into(), (x + inc).into(), time_range, 3))
                .unwrap();
        });
    }
    drop(tx);
    let mut maxval = Values {
        perc_up: 1.0,
        perc_down: 1.0,
        time_down: 3,
        time_up: 3,
        profit: 0.0,
        prices: Vec::new(),
        modified: Vec::new(),
    };
    let val: Vec<Values> = rx.into_iter().collect();
    for z in val {
        if z.profit > maxval.profit {
            maxval = z.clone();
        }
    }
    maxval
}
