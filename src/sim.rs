use crate::model::Values;

pub fn percent(new: f64, old: f64) -> f64 {
    ((new - old) / old) * 100.0
}

pub fn simulate(token: &Values) -> Values {
    let mut token_amount = 0.0;
    let mut fiat_amount = 100.0;
    // for every pair of values
    for i in 0..token.modified.len() {
        if i < token.time_up || i < token.time_down {
            continue;
        }
        let new = token.modified[i];
        let old = token.modified[i - token.time_up];

        let buy = percent(new, old) >= token.perc_up;

        let old = token.modified[i - token.time_down];
        let sell = percent(new, old) <= token.perc_down;

        if buy && sell {
        } else if buy && fiat_amount != 0.0 {
            token_amount = (fiat_amount / token.prices[i]) * 0.999;
            fiat_amount = 0.0;
        } else if sell && token_amount != 0.0 {
            fiat_amount = (token_amount * token.prices[i]) * 0.999;
            token_amount = 0.0;
        } else {
        }
    }
    let account_value: f64 = fiat_amount + (token_amount * token.prices[token.modified.len() - 1]);
    return Values {
        perc_up: token.perc_up,
        perc_down: token.perc_down,
        time_down: token.time_down,
        time_up: token.time_up,
        profit: account_value,
        prices: token.prices.clone(),
        modified: token.modified.clone(),
    };
}
