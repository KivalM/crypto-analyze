use crate::sim::simulate;

use super::model::Values;

pub fn optimum_buy(
    token: Values,
    r1: f64,
    r2: f64,
    time_range: usize,
    decimal_places: u32,
) -> Values {
    let depth = (10 as u32).pow(decimal_places);
    let u1: u32 = (r1 * depth as f64) as u32;
    let u2: u32 = (r2 * depth as f64) as u32;

    let mut maxval = Values {
        perc_up: 1.0,
        perc_down: 1.0,
        time_down: 3,
        time_up: 3,
        profit: 0.0,
        prices: Vec::new(),
        modified: Vec::new(),
    };

    for percup in u1..=u2 {
        for timeup in 1..time_range {
            let perc: f64 = (percup as f64) / depth as f64;
            let val = simulate(&Values {
                perc_up: perc,
                perc_down: token.perc_down,
                time_down: token.time_down,
                time_up: timeup,
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
