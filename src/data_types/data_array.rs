use std::error::Error;
use log::{debug, info};
use std::f64::consts::{E, PI};
use crate::functions::convert::{convert_slice_to_f64, Convert};
use crate::functions::stats_math::standard_deviation;
// use crate::graphing::graph_test;
use crate::logging;

#[derive(Default, Debug, Clone)]
pub struct DataArray {
    pub name: String,
    pub data: Vec<f64>,
    pub population: Option<bool>,
    pub mean: f64,
    pub sum_of_squares: f64,
    pub deviations: Vec<f64>,
    pub variance: f64,
    pub standard_deviation: f64,
    pub z_scores: Vec<f64>,
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
        self.mean = sum / self.data.len() as f64;
    }

    fn calculate_sum_of_squares(&mut self) {
        let mut sum_of_squares_tmp = 0.0;
        for datum in self.data.iter() {
            sum_of_squares_tmp += (datum - self.mean).powi(2);
        }
        self.sum_of_squares = sum_of_squares_tmp;
    }

    fn calculate_deviations(&mut self) {
        let mut deviations_tmp = Vec::new();
        for datum in self.data.iter() {
            deviations_tmp.push(datum - self.mean);
        }
        self.deviations = deviations_tmp;
    }

    fn calculate_variance(&mut self) {
        // N for pop (true), N-1 for sample (default = false)
        self.variance = self.sum_of_squares / (self.data.len() as f64
            - if self.population.unwrap_or_default() { 0.0 } else { 1.0 });
    }

    fn calculate_standard_deviation(&mut self) {
        self.standard_deviation = f64::sqrt(self.variance);
    }

    fn calculate_z_scores(&mut self) {
        let mut z_scores_tmp = Vec::new();
        for datum in self.data.iter() {
            z_scores_tmp.push(datum / self.standard_deviation);
        }
        self.z_scores = z_scores_tmp;
    }

    pub fn get_probability_density(&self, x: f64) -> Result<f64, Box<dyn Error>> {
        let fraction = 1.0 / f64::sqrt(2.0 * PI * self.variance);
        let e_exponential = E.powf(-f64::powi((x - self.mean), 2) / (2.0 * self.variance));
        Ok(fraction * e_exponential)
    }

    // pub fn run_graph_test(&self) {
    //     let mut x_values = Vec::from_iter(0.0..100.0);
    //     x_values.iter().for_each(|&x| self.get_probability_density(*x));
    //
    //     graph_test(String::from("Testing"), x_values).expect("Graphing failed");
    // }

    pub fn print_data(&self) {
        info!("{}", logging::format_title(&*self.name));
        // debug!("Data: {:?}", &self.data);
        info!("N.............................{}", self.data.len());
        info!("Population....................{}", self.population.unwrap_or_default());
        info!("Mean..........................{}", self.mean);
        info!("Sum of Squares................{}", self.sum_of_squares);
        // debug!("Deviations: {:?}", self.deviations.clone().unwrap_or_default());
        info!("Variance......................{}", self.variance);
        info!("Standard deviation............{}", self.standard_deviation);
        // debug!("Z-Scores: {:?}", self.z_scores.clone().unwrap_or_default());
    }
}