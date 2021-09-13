# will make it more user friendy later

for now ~ join discord for help

[Discord](https://discord.gg/eKqScrCUCr)

# Test Set functions

# fn generate

-> gets price data and stores it in data.txt

format is
`'Determinant price', 'CurrencyOfChoice Price'`

# fn simulate

- simulates transactions with given thresholds
- gets data from data.txt
- returns float val containing the percentage increase;

# fn optim_buy

- calls simulate for a values in a range with a specific perc_down and time_down
- will display all the information of simulations that have a higher value than the ones previously displayed

# fn optim_sell

- calls simulate for a values in a range with a specific perc_up and time_up
- will display all the information of simulations that have a higher value than the ones previously displayed

# fn t_sell

- runs optim_sell across multiple(8 but configurable) threads

# fn t_buy

- runs optim_buy across multiple(8 but configurable) threads
