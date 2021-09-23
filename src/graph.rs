use crate::model::Values;

pub fn percent(new: f64, old: f64) -> f64 {
    ((new - old) / old) * 100.0
}

pub fn graphit(token: &Values) {
    let step = 288;
    println!(
        r#"<html>
        <head>
          <script
            type="text/javascript"
            src="https://www.gstatic.com/charts/loader.js"
          ></script>
          <script type="text/javascript">
            google.charts.load("current", {{ packages: ["corechart"] }});
            google.charts.setOnLoadCallback(drawChart);
      
            function drawChart() {{
              var data = google.visualization.arrayToDataTable([
                ["Time", "Ethereum Price", "Account value"],"#
    );
    let mut token_amount = 0.0;
    let mut fiat_amount = token.prices[0];
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
            token_amount = (fiat_amount / token.prices[i]) * 99.9 / 100.0;
            fiat_amount = 0.0;
        } else if sell && token_amount != 0.0 {
            fiat_amount = (token_amount * token.prices[i]) * 99.9 / 100.0;
            token_amount = 0.0;
        } else {
        }
        if i as i32 % step == 0 {
            let account_value: f64 = fiat_amount + (token_amount * token.prices[i]);
            println!("['{}', {}, {}],", i, token.prices[i], account_value);
        }
    }

    println!(
        r#"        ]);
    
            var options = {{
              title: "Binance-auto-trader vs HODLING",
              curveType: "function",
              legend: {{ position: "bottom" }},
            }};
    
            var chart = new google.visualization.LineChart(
              document.getElementById("curve_chart")
            );
    
            chart.draw(data, options);
          }}
        </script>
      </head>
      <body>
        <div id="curve_chart" style="width: 900px; height: 500px"></div>
      </body>
    </html>
    "#
    );
}
