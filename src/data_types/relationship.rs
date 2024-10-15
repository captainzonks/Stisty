use std::iter::Sum;
use anyhow::Error;
use log::info;
use crate::data_types::data_array::DataArray;
use crate::error_types::CSVError;
use crate::functions::convert::Convert;
use crate::logging;

#[derive(Default, Debug)]
pub struct Relationship {
    pub name: String,
    pub data_x_mean: Option<f64>,
    pub data_y_mean: Option<f64>,
    pub sum_of_product_of_z_scores: Option<f64>,
    pub covariance: Option<f64>,
    pub pearson_r: Option<f64>,
    pub degrees_of_freedom: Option<i32>,
    pub t_statistic: Option<f64>,
    pub slope: Option<f64>,
    pub intercept: Option<f64>,
    pub beta_x: Option<f64>,
    pub beta_y: Option<f64>,
}

impl Relationship {
    pub fn new(name: String, data_x: &DataArray, data_y: &DataArray, degrees_of_freedom: Option<i32>) -> Result<Relationship, Error> {
        let mut new_relationship: Relationship = Relationship::default();
        new_relationship.degrees_of_freedom = Some(degrees_of_freedom.unwrap_or(2));

        new_relationship.data_x_mean = Some(data_x.mean.unwrap_or_else(||
            panic!("Data \"{}\" was not valid.\n", data_x.name)
        ));
        new_relationship.data_y_mean = Some(data_y.mean.unwrap_or_else(||
            panic!("Data \"{}\" was not valid.\n", data_y.name)
        ));
        new_relationship.name = name;

        // Sum of Product of Z-Scores
        let zipped = data_x.z_scores.clone().unwrap_or_else(||
            panic!("Data \"{}\" was not valid.\n", data_x.name)
        ).into_iter()
            .zip(data_y.z_scores.clone().unwrap_or_else(||
                panic!("Data \"{}\" was not valid.\n", data_y.name))
                .into_iter());
        let mut growing_products = 0.0;
        for (z_score_x, z_score_y) in zipped {
            growing_products += z_score_x * z_score_y;
        }
        new_relationship.sum_of_product_of_z_scores = Some(growing_products);

        // Covariance = (sum(data_x's deviations * data_y's deviations)) / (N (- 1 if it's a sample, and by default))
        let zipped = data_x.deviations.clone().unwrap().into_iter().zip(data_y.deviations.clone().unwrap().into_iter());
        let mut growing_products = 0.0;
        for (deviation_x, deviation_y) in zipped {
            growing_products += deviation_x * deviation_y;
        }
        new_relationship.covariance = Option::from(growing_products / (data_x.data.len() as f64
            - if data_x.population.unwrap_or_default() { 0.0 } else { 1.0 }));

        // Pearson r = covariance / (data_x's sd * data_y's sd)
        new_relationship.pearson_r =
            Some(new_relationship.covariance.unwrap()
                / (
                data_x.standard_deviation.unwrap_or_else(||
                    panic!("Data \"{}\" was not valid.\n", data_x.name))
                    *
                    data_y.standard_deviation.unwrap_or_else(||
                        panic!("Data \"{}\" was not valid.\n", data_y.name)
                    )
            ));

        // t-statistic = r * sqrt(N - df) / sqrt(1 - r^2)
        new_relationship.t_statistic = Some(new_relationship.pearson_r.unwrap()
            * f64::sqrt(data_x.data.len() as f64 - new_relationship.degrees_of_freedom.unwrap() as f64)
            / f64::sqrt(1.0 - f64::powi(new_relationship.pearson_r.unwrap(), 2)));

        // y-hat = bx + a
        new_relationship.slope = Option::from(new_relationship.covariance.unwrap() / data_x.variance.unwrap());
        new_relationship.intercept = Option::from(data_y.mean.unwrap() - new_relationship.slope.unwrap() * data_x.mean.unwrap());

        Ok(new_relationship)
    }

    pub fn get_y_hat(&self, x_value: f64) -> f64 {
        self.slope.unwrap() * x_value + self.intercept.unwrap()
    }

    pub fn print_relationship(&self) {
        info!("{}", logging::format_title(&*self.name));
        info!("Data X mean...................{}", self.data_x_mean.unwrap_or_default());
        info!("Data Y mean...................{}", self.data_y_mean.unwrap_or_default());
        info!("Sum of Product of Z-Scores....{}", self.sum_of_product_of_z_scores.unwrap_or_default());
        info!("Covariance....................{}", self.covariance.unwrap_or_default());
        info!("Pearson r.....................{}", self.pearson_r.unwrap_or_default());
        info!("t-statistic...................{}", self.t_statistic.unwrap_or_default());
        info!("Slope (b).....................{}", self.slope.unwrap_or_default());
        info!("Intercept (a).................{}", self.intercept.unwrap_or_default());
    }
}
