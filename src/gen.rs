use std::io::Write;
use std::path::Path;

use binance::api::*;
use binance::market::*;

pub fn exists(file: String) -> bool {
    let p = format!("./{}", file);
    Path::new(&p).exists()
}

pub fn generate(pair: &str, startdate: u64, enddate: u64) {
    let market: Market = Binance::new(None, None);
    let mut count = 0;
    let mut val = startdate;
    let mut file = std::fs::File::create(format!("{}.txt", pair)).expect("create failed");
    let mut prices: Vec<f64> = Vec::new();
    let mut mdofied_prices: Vec<f64> = Vec::new();

    while val < enddate {
        val = startdate + (count * 300000000);
        count = count + 1;

        match market.get_klines(pair, "5m", 10000, val, None) {
            Ok(klines) => match klines {
                binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                    for z in klines {
                        let kline: binance::model::KlineSummary = z.clone();
                        prices.push(kline.close);
                        // modify prices here
                        mdofied_prices.push((kline.high + kline.low) / 2.0);
                    }
                }
            },
            Err(e) => println!("Error: {}", e),
        }

        for i in 0..prices.len() {
            let _ = file.write(format!("{},{}\n", &prices[i], &mdofied_prices[i]).as_bytes());
        }
        prices.clear();
        mdofied_prices.clear();
    }
}
