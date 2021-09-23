#[derive(Debug, Clone)]
pub struct Values {
    pub perc_up: f64,
    pub perc_down: f64,
    pub time_down: usize,
    pub time_up: usize,
    pub profit: f64,
    pub prices: Vec<f64>,
    pub modified: Vec<f64>,
}
