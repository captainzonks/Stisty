use anyhow::{Error, Result};
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
    pub fn new<T>(name: String, data: Vec<T>, pop: Option<bool>) -> Result<DataArray, Error>
    where
        T: Copy + std::fmt::Debug,
        f64: Convert<T>,
    {
        let mut new_data_array = DataArray::default();
        new_data_array.name = name;
        new_data_array.data = convert_slice_to_f64(data.as_slice(), 0.0, 1.0)?;
        new_data_array.population = pop;
        new_data_array.run_calculations();
        Ok(new_data_array)
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

    // mean = sum(x_i) / N
    fn calculate_mean(&mut self) {
        self.mean = self.data.iter().sum::<f64>() / self.data.len() as f64;
    }

    // ss = sum((x_i - mean)^2)
    fn calculate_sum_of_squares(&mut self) {
        self.sum_of_squares = self.data.iter()
            .map(|x| f64::powi(x - self.mean, 2))
            .sum::<f64>();
    }

    fn calculate_deviations(&mut self) {
        self.deviations = self.data.iter()
            .map(|x| x - self.mean).collect();
    }

    // s^2 = ss / (N - 1)
    fn calculate_variance(&mut self) {
        // N for pop (true), N-1 for sample (default = false)
        self.variance = self.sum_of_squares / (self.data.len() as f64
            - if self.population.unwrap_or_default() { 0.0 } else { 1.0 });
    }

    // s = sqrt(s^2)
    fn calculate_standard_deviation(&mut self) {
        self.standard_deviation = f64::sqrt(self.variance);
    }

    // z = x / s
    fn calculate_z_scores(&mut self) {
        self.z_scores = self.data.iter()
            .map(|x| x / self.standard_deviation).collect();
    }

    pub fn get_probability_density(&self, x: f64) -> Result<f64, Error> {
        let fraction = 1.0 / f64::sqrt(2.0 * PI * self.variance);
        let e_exponential = E.powf(-f64::powi((x - self.mean), 2) / (2.0 * self.variance));
        Ok(fraction * e_exponential)
    }

    // raw = deviation + mean
    pub fn get_raw_scores_from_deviations(&self) -> Result<Vec<f64>, Error> {
        Ok(
            self.deviations.iter()
            .map(|x| *x + self.mean).collect()
        )
    }

    pub fn get_single_t(&self, mu: f64) -> Result<f64, Error> {
        Ok((self.mean - mu) / (self.standard_deviation / f64::sqrt(self.data.len() as f64)))
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