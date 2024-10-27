use std::iter::Sum;
use anyhow::{Error, Result};
use log::info;
use crate::data_types::data_array::DataArray;
use crate::error_types::CSVError;
use crate::functions::convert::Convert;
use crate::functions::stats_math::mean;
use crate::logging;

#[derive(Default, Debug, Clone)]
pub struct Relationship {
    pub name: String,
    pub n: i32,
    pub p: i32,

    pub data_x: DataArray,
    pub data_y: DataArray,

    pub sum_of_product_of_z_scores: f64,
    pub sum_of_product_of_deviations: f64,
    pub covariance: f64,
    pub pearson_r: f64,
    pub t_score: f64,
    pub slope_beta: f64,
    pub slope_beta_hat: f64, // unbiased
    pub intercept_alpha: f64,
    pub intercept_alpha_hat: f64, // unbiased
    pub observed_values: Vec<f64>, // y_i
    pub fitted_values: Vec<f64>, // y-hat
    pub residuals: Vec<f64>, // y_i - y-hat
    pub sum_of_squares_total: f64, // SST
    pub sum_of_squares_error: f64, // SSE
    pub explained_sum_of_squares: f64, // ESS
    pub coefficient_of_multiple_determination: f64, // R^2

    // R^2 = proportion of observed y variation that can be explained by the simple linear regression model
}

impl Relationship {
    pub fn new(name: String, data_x: &DataArray, data_y: &DataArray) -> Result<Relationship, Error> {
        let mut new_relationship: Relationship = Relationship::default();
        new_relationship.name = name;
        new_relationship.n = data_x.data.len() as i32;

        new_relationship.data_x = data_x.clone();
        new_relationship.data_y = data_y.clone();

        // Sum of Product of Z-Scores
        let mut zipped = new_relationship.data_x.z_scores.clone().into_iter()
            .zip(new_relationship.data_y.z_scores.clone().into_iter());
        for (z_score_x, z_score_y) in zipped {
            new_relationship.sum_of_product_of_z_scores += z_score_x * z_score_y;
        }

        // Sum of Product of Deviations
        zipped = new_relationship.data_x.deviations.clone().into_iter()
            .zip(new_relationship.data_y.deviations.clone().into_iter());
        for (deviation_x, deviation_y) in zipped {
            new_relationship.sum_of_product_of_deviations += deviation_x * deviation_y;
        }

        // Covariance = (sum(data_x's deviations * data_y's deviations)) / (N (- 1, for Bessel's Correction))
        new_relationship.covariance = new_relationship.sum_of_product_of_deviations
            / (new_relationship.data_x.data.len() as f64
            - if new_relationship.data_x.population.unwrap_or_default() { 0.0 } else { 1.0 });

        // Pearson r = covariance / (data_x's sd * data_y's sd)
        new_relationship.pearson_r = new_relationship.covariance
            / (new_relationship.data_x.standard_deviation * data_y.standard_deviation);

        // t-score (from Pearson r) = r * sqrt(N - 2) / sqrt(1 - r^2)
        new_relationship.t_score = new_relationship.pearson_r
            * f64::sqrt(new_relationship.data_x.data.len() as f64 - 2.0)
            / f64::sqrt(1.0 - f64::powi(new_relationship.pearson_r, 2));

        // y-hat = beta(x) + alpha
        // x = (y-hat - alpha) / beta
        // beta = (y-hat - alpha) / x
        // alpha = y-hat - (beta)x

        new_relationship.slope_beta = new_relationship.covariance / new_relationship.data_x.variance;

        new_relationship.intercept_alpha = new_relationship.data_y.mean
            - new_relationship.slope_beta * new_relationship.data_x.mean;

        // beta_hat = sum((x_i - x-bar)(y - y-bar)) / sum((x_i - x-bar)^2)
        new_relationship.slope_beta_hat = new_relationship.sum_of_product_of_deviations
            / new_relationship.data_x.sum_of_squares;

        // alpha_hat = y-bar - (beta_hat * x-bar)
        new_relationship.intercept_alpha_hat = new_relationship.data_y.mean
            - (new_relationship.slope_beta_hat * new_relationship.data_x.mean);

        // calculate fitted
        for datum in new_relationship.data_x.data.clone().into_iter() {
            new_relationship.fitted_values.push(
                new_relationship.get_y_hat(datum)
            );
        }

        // calculate residuals and get SST and SSE
        zipped = new_relationship.fitted_values.clone().into_iter().zip(new_relationship.data_y.data.clone().into_iter());
        for (predicted_y, observed_y) in zipped {
            new_relationship.residuals.push(observed_y - predicted_y);
            new_relationship.observed_values.push(observed_y);
            new_relationship.sum_of_squares_total += f64::powi(observed_y - new_relationship.data_y.mean, 2);
            new_relationship.sum_of_squares_error += f64::powi(observed_y - predicted_y, 2);
            new_relationship.explained_sum_of_squares += f64::powi(predicted_y - new_relationship.data_y.mean, 2);
        }

        // ESS, cheaper method (and perhaps not completely accurate)
        // new_relationship.explained_sum_of_squares = new_relationship.sum_of_squares_total - new_relationship.sum_of_squares_error;

        // coefficient of multiple determination, or R^2 (ESS/SST)
        new_relationship.coefficient_of_multiple_determination = new_relationship.explained_sum_of_squares
            / new_relationship.sum_of_squares_total;


        Ok(new_relationship)
    }

    pub fn get_y_hat(&self, x_value: f64) -> f64 {
        self.slope_beta * x_value + self.intercept_alpha
    }

    pub fn get_x(&self, y_value: f64) -> f64 {
        (y_value - self.intercept_alpha) / self.slope_beta
    }

    pub fn get_intercept_alpha(&self, y_value: f64, x_value: f64) -> f64 {
        y_value - self.intercept_alpha * x_value
    }

    pub fn get_slope_beta(&self, y_value: f64, x_value: f64) -> f64 {
        (y_value - self.intercept_alpha) / x_value
    }

    pub fn print_relationship(&self) {
        info!("{}", logging::format_title(&*self.name));
        info!("n................................{}", self.n);
        info!("Data X mean......................{}", self.data_x.mean);
        info!("Data Y mean......................{}", self.data_y.mean);
        info!("Sum of Product of Z-Scores.......{}", self.sum_of_product_of_z_scores);
        info!("Sum of Product of Deviations.....{}", self.sum_of_product_of_deviations);
        info!("Covariance.......................{}", self.covariance);
        info!("Pearson r........................{}", self.pearson_r);
        info!("t-score..........................{}", self.t_score);
        info!("Slope (Beta).....................{}", self.slope_beta);
        info!("Estimated Slope (Beta-hat).......{}", self.slope_beta_hat);
        info!("Intercept (Alpha)................{}", self.intercept_alpha);
        info!("Estimated Intercept (Alpha-hat)..{}", self.intercept_alpha_hat);
        info!("Observed Values (Y_i)............{:?}", self.observed_values);
        info!("Fitted Values (Y-hat)............{:?}", self.fitted_values);
        info!("Residuals (Y_i - Y-hat)..........{:?}", self.residuals);
        info!("Sum of Squared Totals............{}", self.sum_of_squares_total);
        info!("Sum of Squared Errors............{}", self.sum_of_squares_error);
        info!("Explained Sum of Squares.........{}", self.explained_sum_of_squares);
        info!("R^2..............................{}", self.coefficient_of_multiple_determination);
        info!("{}", logging::format_title(""));
    }
}
