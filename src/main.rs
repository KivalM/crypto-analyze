pub mod gen;
pub mod read;
pub mod rund;
pub mod threaded;

use std::time::{SystemTime, UNIX_EPOCH};

use threaded::t_sell;

use crate::{gen::generate, read::readtovec, rund::simulate, threaded::t_buy};
const DETERMINANT: &str = "ETHUSDT";
const FIAT: &str = "USDT";
const CURRENCY: &str = "ETH";
const PAIR: &str = "ETHUSDT";

pub fn run(determinant: &str, pair: &str, startdate: u64) {
    // let start = SystemTime::now();
    // let since_the_epoch = start
    //     .duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards");
    // let in_ms =
    //     since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;

    // gen::generate(determinant, pair, startdate, in_ms);

    let (det, curr) = readtovec();
    // generate(determinant, pair, startdate, in_ms);
    // // simulaute with given values;
    // println!("{}", simulate(-1.15, 168, 0.16, 151, det, curr));

    // // simulate buying optimisation in range r1..r2 with 8 threads
    // // lower percent is = to r1/100;
    // // upper percent is = to r2/100;
    // // r1 == 10x percentage that you want
    //VAL: 558.6764248913611 time_d: 50 perc_d: -1 time_u: 145 perc_u 0.344
    //VAL: 884.5017910918306 time_d: 149 perc_d: -1.475 time_u: 145 perc_u 0.344
    //VAL: 1263.3241210618228 time_d: 149 perc_d: -1.475 time_u: 143 perc_u 1.552
    //VAL: 1411.6712146134594 time_d: 146 perc_d: -1.781 time_u: 143 perc_u 1.552

    t_buy(0, 2000, -1.781, 288, 146);

    //t_sell(-2000, 0, 1.552, 143, 288);
}

fn main() {
    run("ADAUSDT", "ADAUSDT", 1609452000000);
}
