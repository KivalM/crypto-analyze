use std::io::Write;

use binance::api::*;
use binance::market::*;

pub fn generate(determinant: &str, pair: &str, startdate: u64, enddate: u64) {
    let market: Market = Binance::new(None, None);
    let mut count = 0;
    let mut val = startdate;
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    let mut det: Vec<f64> = Vec::new();
    let mut curr: Vec<f64> = Vec::new();

    while val < enddate {
        val = startdate + (count * 300000000);
        count = count + 1;
        match market.get_klines(determinant, "5m", 10000, val, None) {
            Ok(klines) => {
                match klines {
                    binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                        for z in klines {
                            let kline: binance::model::KlineSummary = z.clone(); // You need to iterate over the klines
                            det.push(kline.open);
                        }
                        println!("{}", "RIP");
                    }
                }
            }
            Err(e) => println!("Error: {}", e),
        }

        match market.get_klines(pair, "5m", 10000, val, None) {
            Ok(klines) => {
                match klines {
                    binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                        for z in klines {
                            let kline: binance::model::KlineSummary = z.clone(); // You need to iterate over the klines
                            curr.push(kline.open);
                        }
                        println!("{}", "RIP");
                    }
                }
            }
            Err(e) => println!("Error: {}", e),
        }

        for i in 0..curr.len() {
            let _ = file.write(format!("{},{}\n", &curr[i], &det[i]).as_bytes());
        }
        det.clear();
        curr.clear();
    }
}
