use std::thread::{self, JoinHandle};

use crate::{read::readtovec, rund::optim_sell};

use super::rund::optim_buy;

pub fn t_sell(r1: i32, r2: i32, perc_up: f64, time_up: usize, time_d: usize) {
    let threads = 8;
    let range = r2 - r1;
    let inc = range / threads;
    let (det, curr) = readtovec();
    // ownership error so have to roll out the loop
    // let mut handles: Vec<JoinHandle<()>> = Vec::new();
    // for i in 0..threads {
    //     let d1 = r1 + (i * inc);
    //     let d2 = r1 + ((i + 1) * inc);
    //     let d = det.clone();
    //     let c = curr.clone();
    //     handles.push(thread::spawn(move || {
    //         optim_sell(perc_up, time_up, d1, d2, time_d, d, c)
    //     }))
    // }

    let mut i = 0;
    let d1 = r1 + (i * inc);
    let d2 = r1 + ((i + 1) * inc);
    println!("running from {} to {}", d1, d2);
    let d = det.clone();
    let c = curr.clone();
    let handle1 = thread::spawn(move || optim_sell(perc_up, time_up, d1, d2, time_d, d, c));
    i = i + 1;

    let d1 = r1 + (i * inc);
    let d2 = r1 + ((i + 1) * inc);
    println!("running from {} to {}", d1, d2);
    let d = det.clone();
    let c = curr.clone();
    let handle2 = thread::spawn(move || optim_sell(perc_up, time_up, d1, d2, time_d, d, c));
    i = i + 1;

    let d1 = r1 + (i * inc);
    let d2 = r1 + ((i + 1) * inc);
    println!("running from {} to {}", d1, d2);
    let d = det.clone();
    let c = curr.clone();
    let handle3 = thread::spawn(move || optim_sell(perc_up, time_up, d1, d2, time_d, d, c));
    i = i + 1;

    let d1 = r1 + (i * inc);
    let d2 = r1 + ((i + 1) * inc);
    println!("running from {} to {}", d1, d2);
    let d = det.clone();
    let c = curr.clone();
    let handle4 = thread::spawn(move || optim_sell(perc_up, time_up, d1, d2, time_d, d, c));
    i = i + 1;

    let d1 = r1 + (i * inc);
    let d2 = r1 + ((i + 1) * inc);
    println!("running from {} to {}", d1, d2);
    let d = det.clone();
    let c = curr.clone();
    let handle5 = thread::spawn(move || optim_sell(perc_up, time_up, d1, d2, time_d, d, c));
    i = i + 1;

    let d1 = r1 + (i * inc);
    let d2 = r1 + ((i + 1) * inc);
    println!("running from {} to {}", d1, d2);
    let d = det.clone();
    let c = curr.clone();
    let handle6 = thread::spawn(move || optim_sell(perc_up, time_up, d1, d2, time_d, d, c));
    i = i + 1;

    let d1 = r1 + (i * inc);
    let d2 = r1 + ((i + 1) * inc);
    println!("running from {} to {}", d1, d2);
    let d = det.clone();
    let c = curr.clone();
    let handle7 = thread::spawn(move || optim_sell(perc_up, time_up, d1, d2, time_d, d, c));
    i = i + 1;

    let d1 = r1 + (i * inc);
    let d2 = r1 + ((i + 1) * inc);
    println!("running from {} to {}", d1, d2);
    let d = det.clone();
    let c = curr.clone();
    let handle8 = thread::spawn(move || optim_sell(perc_up, time_up, d1, d2, time_d, d, c));

    handle8.join().unwrap();
}

pub fn t_buy(r1: i32, r2: i32, perc_down: f64, time_up: usize, time_d: usize) {
    let threads = 8;
    let range = r2 - r1;
    let inc = range / threads;
    let (det, curr) = readtovec();
    let mut i = 0;
    // ownership error so have to roll out the loop
    // for i in 0..threads {
    //     let d1 = r1 + (i * inc);
    //     let d2 = r1 + ((i + 1) * inc);
    //     let d = det.clone();
    //     let c = curr.clone();
    //     let handle = thread::spawn(move || optim_buy(perc_down, time_d, d1, d2, time_up, d, c));
    //     handles.push(handle);
    //     if i == threads {
    //         handle.join();
    //     }
    // }
    let d1 = r1 + (i * inc);
    let d2 = r1 + ((i + 1) * inc);
    println!("running from {} to {}", d1, d2);
    let d = det.clone();
    let c = curr.clone();
    let handle1 = thread::spawn(move || optim_buy(perc_down, time_d, d1, d2, time_up, d, c));
    i = i + 1;

    let d1 = r1 + (i * inc);
    let d2 = r1 + ((i + 1) * inc);
    println!("running from {} to {}", d1, d2);
    let d = det.clone();
    let c = curr.clone();
    let handle2 = thread::spawn(move || optim_buy(perc_down, time_d, d1, d2, time_up, d, c));
    i = i + 1;

    let d1 = r1 + (i * inc);
    let d2 = r1 + ((i + 1) * inc);
    println!("running from {} to {}", d1, d2);
    let d = det.clone();
    let c = curr.clone();
    let handle3 = thread::spawn(move || optim_buy(perc_down, time_d, d1, d2, time_up, d, c));
    i = i + 1;

    let d1 = r1 + (i * inc);
    let d2 = r1 + ((i + 1) * inc);
    println!("running from {} to {}", d1, d2);
    let d = det.clone();
    let c = curr.clone();
    let handle4 = thread::spawn(move || optim_buy(perc_down, time_d, d1, d2, time_up, d, c));
    i = i + 1;

    let d1 = r1 + (i * inc);
    let d2 = r1 + ((i + 1) * inc);
    println!("running from {} to {}", d1, d2);
    let d = det.clone();
    let c = curr.clone();
    let handle5 = thread::spawn(move || optim_buy(perc_down, time_d, d1, d2, time_up, d, c));
    i = i + 1;

    let d1 = r1 + (i * inc);
    let d2 = r1 + ((i + 1) * inc);
    println!("running from {} to {}", d1, d2);
    let d = det.clone();
    let c = curr.clone();
    let handle6 = thread::spawn(move || optim_buy(perc_down, time_d, d1, d2, time_up, d, c));
    i = i + 1;
    let d1 = r1 + (i * inc);
    let d2 = r1 + ((i + 1) * inc);
    println!("running from {} to {}", d1, d2);
    let d = det.clone();
    let c = curr.clone();
    let handle7 = thread::spawn(move || optim_buy(perc_down, time_d, d1, d2, time_up, d, c));
    i = i + 1;
    let d1 = r1 + (i * inc);
    let d2 = r1 + ((i + 1) * inc);
    println!("running from {} to {}", d1, d2);
    let d = det.clone();
    let c = curr.clone();
    let handle8 = thread::spawn(move || optim_buy(perc_down, time_d, d1, d2, time_up, d, c));

    handle7.join().unwrap();
}
