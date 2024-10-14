use log::{debug, info};
use crate::functions::convert::{convert_slice_to_f64, Convert};
use crate::logging;

#[derive(Default, Debug, Clone)]
pub struct DataArray {
    pub name: String,
    pub data: Vec<f64>,
    pub population: Option<bool>,
    pub mean: Option<f64>,
    pub sum_of_squares: Option<f64>,
    pub deviations: Option<Vec<f64>>,
    pub variance: Option<f64>,
    pub standard_deviation: Option<f64>,
    pub z_scores: Option<Vec<f64>>,
}

impl DataArray {
    pub fn new<T>(name: String, data: Vec<T>, pop: Option<bool>) -> DataArray
    where
        T: Copy + std::fmt::Debug,
        f64: Convert<T>,
    {
        let mut new_data_array = DataArray::default();
        new_data_array.name = name;
        new_data_array.data = convert_slice_to_f64(data.as_slice(), 0.0, 1.0);
        new_data_array.population = pop;
        new_data_array.run_calculations();
        new_data_array
    }

    pub fn run_calculations(&mut self) {
        // Mean
        self.calculate_mean();

        // Sum of Squares
        self.calculate_sum_of_squares();

        // Deviations
        self.calculate_deviations();

        // Variance
        self.calculate_variance();

        // Standard Deviation
        self.calculate_standard_deviation();

        // Z-Scores
        self.calculate_z_scores();
    }

    fn calculate_mean(&mut self) {
        let mut sum = 0.0;
        for datum in self.data.iter() {
            sum += datum;
        };
        self.mean = Some(sum / self.data.len() as f64);
    }

    fn calculate_sum_of_squares(&mut self) {
        let mut sum_of_squares_tmp = 0.0;
        for datum in self.data.iter() {
            sum_of_squares_tmp += (datum - self.mean.unwrap()).powi(2);
        }
        self.sum_of_squares = Some(sum_of_squares_tmp);
    }

    fn calculate_deviations(&mut self) {
        let mut deviations_tmp = Vec::new();
        for datum in self.data.iter() {
            deviations_tmp.push(datum - self.mean.unwrap());
        }
        self.deviations = Some(deviations_tmp);
    }

    fn calculate_variance(&mut self) {
        // N for pop (true), N-1 for sample (default = false)
        self.variance = Option::from(self.sum_of_squares.unwrap() /
            (self.data.len() as f64 -
                if self.population.unwrap_or_default() { 0.0 } else { 1.0 }));
    }

    fn calculate_standard_deviation(&mut self) {
        self.standard_deviation = Option::from(f64::sqrt(self.variance.unwrap()));
    }

    fn calculate_z_scores(&mut self) {
        let mut z_scores_tmp = Vec::new();
        for datum in self.data.iter() {
            z_scores_tmp.push(datum / self.standard_deviation.unwrap());
        }
        self.z_scores = Some(z_scores_tmp);
    }

    pub fn print_data(&self) {
        info!("{}", logging::format_title(&*self.name));
        // debug!("Data: {:?}", &self.data);
        info!("Population....................{}", self.population.unwrap_or_default());
        info!("Mean..........................{}", self.mean.unwrap_or_default());
        info!("Sum of Squares................{}", self.sum_of_squares.unwrap_or_default());
        // debug!("Deviations: {:?}", self.deviations.clone().unwrap_or_default());
        info!("Variance......................{}", self.variance.unwrap_or_default());
        info!("Standard deviation............{}", self.standard_deviation.unwrap_or_default());
        // debug!("Z-Scores: {:?}", self.z_scores.clone().unwrap_or_default());
    }
}