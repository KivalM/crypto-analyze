use std::convert::TryInto;

use crate::{model::Values, sim::simulate};

pub fn optimum_sell(
    token: Values,
    r1: f64,
    r2: f64,
    time_range: usize,
    decimal_places: i64,
) -> Values {
    let depth = (10 as i64).pow(decimal_places.try_into().unwrap());
    let u1: i64 = (r1 * depth as f64) as i64;
    let u2: i64 = (r2 * depth as f64) as i64;
    // println!("{}", u1);
    // println!("{}", u2);

    let mut maxval = Values {
        perc_up: 1.0,
        perc_down: 1.0,
        time_down: 3,
        time_up: 3,
        profit: 0.0,
        prices: Vec::new(),
        modified: Vec::new(),
    };

    for percdown in u1..=u2 {
        for timedown in 1..time_range {
            let perc: f64 = (percdown as f64) / depth as f64;
            let val = simulate(&Values {
                perc_up: token.perc_up,
                perc_down: perc,
                time_down: timedown,
                time_up: token.time_up,
                profit: 0.0,
                prices: token.prices.clone(),
                modified: token.modified.clone(),
            });
            if val.profit > maxval.profit {
                maxval = val.clone();
                // println!("{:?}", val);
            }
        }
    }

    maxval
}
