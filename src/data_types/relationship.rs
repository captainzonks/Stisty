use std::iter::Sum;
use anyhow::Error;
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
    pub coefficients: i32,
    pub data_x_mean: f64,
    pub data_y_mean: f64,
    pub sum_of_product_of_z_scores: f64,
    pub sum_of_product_of_deviations: f64,
    pub covariance: f64,
    pub pearson_r: f64,
    pub degrees_of_freedom: i32,
    pub t_statistic: f64,
    pub slope: f64,
    pub intercept: f64,
    pub fitted_values: Vec<f64>,
    pub residuals: Vec<f64>,
    pub sum_of_squared_totals: f64, // SST
    pub sum_of_squared_errors: f64, // SSE
    pub sum_of_squared_regression: f64, // SSR
    pub coefficient_of_multiple_determination: f64, // R^2

    // R^2 = proportion of observed y variation that can be explained by the simple linear regression model

    // pub coefficient_of_multiple_determination_adjusted: f64, // Ra^2
}

impl Relationship {
    pub fn new(name: String, data_x: &DataArray, data_y: &DataArray, degrees_of_freedom: Option<i32>) -> Result<Relationship, Error> {
        let mut new_relationship: Relationship = Relationship::default();
        new_relationship.name = name;
        new_relationship.n = data_x.data.len() as i32;
        new_relationship.degrees_of_freedom = degrees_of_freedom.unwrap_or(2);

        new_relationship.data_x_mean = data_x.mean;
        new_relationship.data_y_mean = data_y.mean;

        // Sum of Product of Z-Scores
        let mut zipped = data_x.z_scores.clone().into_iter()
            .zip(data_y.z_scores.clone().into_iter());
        let mut growing_products = 0.0;
        for (z_score_x, z_score_y) in zipped {
            growing_products += z_score_x * z_score_y;
        }
        new_relationship.sum_of_product_of_z_scores = growing_products;

        // Sum of Product of Deviations
        zipped = data_x.deviations.clone().into_iter()
            .zip(data_y.deviations.clone().into_iter());
        growing_products = 0.0;
        for (deviation_x, deviation_y) in zipped {
            growing_products += deviation_x * deviation_y;
        }
        new_relationship.sum_of_product_of_deviations = growing_products;

        // Covariance = (sum(data_x's deviations * data_y's deviations)) / (N (- 1 if it's a sample, and by default))
        zipped = data_x.deviations.clone().into_iter()
            .zip(data_y.deviations.clone().into_iter());
        growing_products = 0.0;
        for (deviation_x, deviation_y) in zipped {
            growing_products += deviation_x * deviation_y;
        }
        new_relationship.covariance = growing_products / (data_x.data.len() as f64
            - if data_x.population.unwrap_or_default() { 0.0 } else { 1.0 });

        // Pearson r = covariance / (data_x's sd * data_y's sd)
        new_relationship.pearson_r = new_relationship.covariance
            / (data_x.standard_deviation * data_y.standard_deviation);

        // t-statistic = r * sqrt(N - df) / sqrt(1 - r^2)
        new_relationship.t_statistic = new_relationship.pearson_r
            * f64::sqrt(data_x.data.len() as f64 - new_relationship.degrees_of_freedom as f64)
            / f64::sqrt(1.0 - f64::powi(new_relationship.pearson_r, 2));

        // y-hat = bx + a
        // x = (y-hat - a) / b
        // b = (y-hat - a) / x
        // a = y-hat - bx
        new_relationship.slope = new_relationship.covariance / data_x.variance;
        new_relationship.intercept = data_y.mean - new_relationship.slope * data_x.mean;

        // calculate fitted
        for datum in data_x.data.clone().into_iter() {
            new_relationship.fitted_values.push(
                new_relationship.get_y_hat(datum)
            );
        }

        // calculate residuals and get SST and SSE
        zipped = new_relationship.fitted_values.clone().into_iter().zip(data_y.data.clone().into_iter());
        for (predicted_y, observed_y) in zipped {
            new_relationship.residuals.push(observed_y - predicted_y);
            new_relationship.sum_of_squared_totals += f64::powi(observed_y - new_relationship.data_y_mean, 2);
            new_relationship.sum_of_squared_errors += f64::powi(observed_y - predicted_y, 2);
            // new_relationship.sum_of_squared_regression += f64::powi(predicted_y - new_relationship.data_y_mean, 2);
        }

        // SSR, cheaper method
        new_relationship.sum_of_squared_regression = new_relationship.sum_of_squared_totals - new_relationship.sum_of_squared_errors;

        // coefficient of multiple determination, or R^2 (SSR/SST)
        new_relationship.coefficient_of_multiple_determination = new_relationship.sum_of_squared_regression
            / new_relationship.sum_of_squared_totals;

        // new_relationship.coefficient_of_multiple_determination_adjusted = 1 - (new_relationship.n - 1) / (new_relationship.n - ());

        Ok(new_relationship)
    }

    pub fn get_y_hat(&self, x_value: f64) -> f64 {
        self.slope * x_value + self.intercept
    }

    pub fn get_x(&self, y_value: f64) -> f64 {
        (y_value - self.intercept) / self.slope
    }

    pub fn get_intercept(&self, y_value: f64, x_value: f64) -> f64 {
        (y_value - self.intercept) / x_value
    }

    pub fn get_slope(&self, y_value: f64, x_value: f64) -> f64 {
        y_value - (self.intercept / x_value)
    }

    pub fn print_relationship(&self) {
        info!("{}", logging::format_title(&*self.name));
        info!("N.............................{}", self.n);
        info!("Data X mean...................{}", self.data_x_mean);
        info!("Data Y mean...................{}", self.data_y_mean);
        info!("Sum of Product of Z-Scores....{}", self.sum_of_product_of_z_scores);
        info!("Sum of Product of Deviations..{}", self.sum_of_product_of_deviations);
        info!("Covariance....................{}", self.covariance);
        info!("Pearson r.....................{}", self.pearson_r);
        info!("t-statistic...................{}", self.t_statistic);
        info!("Slope (Beta1).................{}", self.slope);
        info!("Y-Intercept (Beta0)...........{}", self.intercept);
        info!("Fitted Values.................{:?}", self.fitted_values);
        info!("Residuals.....................{:?}", self.residuals);
        info!("Sum of Squared Totals.........{}", self.sum_of_squared_totals);
        info!("Sum of Squared Errors.........{}", self.sum_of_squared_errors);
        info!("Sum of Squared Regression.....{}", self.sum_of_squared_regression);
        info!("R^2...........................{}", self.coefficient_of_multiple_determination);
        info!("{}", logging::format_title(""));
    }
}
