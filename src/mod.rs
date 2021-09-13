pub mod gen;
pub mod read;
pub mod rund;
pub mod threaded;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::test::{gen::generate, threaded::t_sell};

use self::{read::readtovec, rund::simulate, threaded::t_buy};

pub fn run(determinant: &str, pair: &str, startdate: u64) {
    // let start = SystemTime::now();
    // let since_the_epoch = start
    //     .duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards");
    // let in_ms =
    //     since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;

    // gen::generate(determinant, pair, startdate, in_ms);

    // let (det, curr) = readtovec();
    // generate(determinant, pair, startdate, in_ms);
    // simulaute with given values;
    // println!("{}", simulate(-1.15, 168, 0.16, 151, det, curr));

    // simulate buying optimisation in range r1..r2 with 8 threads
    // lower percent is = to r1/100;
    // upper percent is = to r2/100;
    // r1 == 10x percentage that you want
    //t_buy(0, 2000, -1.866, 288, 22);
    //
    t_sell(-2000, 0, 0.865, 151, 288);
}
