use log::info;
use crate::functions::{convert, stats_math};
use crate::functions::convert::Convert;

const MODULE_NAME: &str = "STATS_MATH";

pub fn mean<T: Copy>(data: &Vec<T>) -> f64
where
    f64: Convert<T>,
{
    let inner_data = convert::convert_slice_to_f64(data, 0.0, 1.0);

    let mut sum = 0.0;

    for datum in inner_data.iter() {
        sum += datum;
    };

    let mean = sum / inner_data.len() as f64;
    info!("{}: Calculated mean ({}) from provided data", MODULE_NAME, mean);
    mean
}

pub fn sum_of_squares<T: Copy>(data: &Vec<T>) -> f64
where
    f64: Convert<T>,
{
    let inner_data = convert::convert_slice_to_f64(data, 0.0, 1.0);
    let mut sum_of_squares = 0.0;

    let mean = mean(data);

    for datum in inner_data.iter() {
        sum_of_squares += (datum - mean).powi(2);
    }

    sum_of_squares
}

pub fn deviation<T: Copy>(datum: T, data: &Vec<T>) -> f64
where
    f64: Convert<T>,
{
    f64::convert(datum) - mean(data)
}

pub fn variance<T: Copy>(data: &Vec<T>, pop: Option<bool>) -> f64
where
    f64: Convert<T>,
{
    let sum_of_squares = sum_of_squares::<T>(data);
    sum_of_squares / (data.len() as f64 - if pop.unwrap_or_default() { 0.0 } else { 1.0 }) // N for pop (true), N-1 for sample (default = false)
}

pub fn standard_deviation<T: Copy>(data: Option<&Vec<T>>, variance: Option<f64>, pop: Option<bool>) -> f64
where
    f64: Convert<T>,
{
    f64::sqrt(match (data, variance) {
        (Some(data), None) => {
            stats_math::variance(data, pop)
        }
        (None, Some(variance)) => {
            variance
        }
        (_, Some(variance)) => {
            variance
        }
        (None, None) => {
            panic!("No valid data provided to calculate the standard deviation.")
        }
    })
}

pub fn approx_standard_deviation<T, U, V>(n: T, p: U, q: V) -> f64
where
    f64: Convert<T>,
    f64: Convert<U>,
    f64: Convert<V>,
{
    f64::sqrt(f64::convert(n) * f64::convert(p) * f64::convert(q))
}

pub fn z_score<T: Copy + std::fmt::Display, U: Copy>(datum: Option<T>,
                                                     deviation: Option<f64>,
                                                     data: Option<&Vec<U>>,
                                                     data_mean: Option<f64>,
                                                     sd: Option<f64>,
                                                     pop: Option<bool>) -> f64
where
    f64: Convert<T>,
    f64: Convert<U>,
{
    match (datum, deviation, data, data_mean, sd, pop) {
        (None, None, None, None, None, None) => panic!("No data!"),
        (Some(datum), _, Some(data), _, _, _) => {
            info!("{}: Calculating z-score from provided datum ({}) and data", MODULE_NAME, datum);
            (f64::convert(datum) - mean(data)) / standard_deviation(Some(data), None, pop)
        }
        (Some(datum), _, _, Some(data_mean), Some(sd), _) => {
            info!("{}: Calculating z-score from provided datum ({}) and mean ({}) and standard deviation ({})", MODULE_NAME, datum, data_mean, sd);
            (f64::convert(datum) - data_mean) / sd
        }
        (_, Some(deviation), Some(data), _, _, _) => {
            info!("{}: Calculating z-score from provided deviation ({}) and data", MODULE_NAME, deviation);
            deviation / standard_deviation(Some(data), None, pop)
        }
        (_, Some(deviation), _, _, Some(sd), _) => {
            info!("{}: Calculating z-score from provided deviation ({}) and standard deviation ({})", MODULE_NAME, deviation, sd);
            deviation / sd
        }
        _ => {
            panic!("Z-Score failed to calculate!")
        }
    }
}

pub fn z_score_from_deviation<T: Copy, U: Copy>(deviation: T, data: &Vec<U>, pop: Option<bool>) -> f64
where
    f64: Convert<T>,
    f64: Convert<U>,
{
    f64::convert(deviation) / standard_deviation(Some(data), None, pop)
}

pub fn z_score_from_raw<T: Copy>(datum: T, data: &Vec<T>, pop: Option<bool>) -> f64
where
    f64: Convert<T>,
{
    (f64::convert(datum) - mean(data)) / standard_deviation(Some(data), None, pop)
}

pub fn z_score_from_normal_approximation<T: Copy>(x: T, n: T, p: T, q: T) -> f64
where
    f64: Convert<T>,
{
    (f64::convert(x) - (f64::convert(n) * f64::convert(p))) / (f64::sqrt(f64::convert(n) * f64::convert(p) * f64::convert(q)))
}

pub fn x_from_znpq<T: Copy>(z: T, n: T, p: T, q: T) -> f64
where
    f64: Convert<T>,
{
    f64::convert(z) * f64::sqrt(f64::convert(n) * f64::convert(p) * f64::convert(q)) + f64::convert(n) * f64::convert(p)
    // z * sqrt(n * p * q) + n * p
}

pub fn raw_score_from_z_data<T: Copy, U: Copy>(z: T, data: &Vec<U>, pop: Option<bool>) -> f64
where
    f64: Convert<T>,
    f64: Convert<U>,
{
    mean(data) + standard_deviation(Some(data), None, pop) * f64::convert(z)
}

pub fn raw_score_from_z_mean_sd<T: Copy>(z: T, data_mean: f64, data_sd: f64) -> f64
where
    f64: Convert<T>,
{
    data_mean + data_sd * f64::convert(z)
}

pub fn covariance<T: Copy>(data_x: &Vec<T>, data_y: &Vec<T>) -> f64
where
    f64: Convert<T>,
{
    let mean_x = mean(data_x);
    let mean_y = mean(data_y);

    let zipped = data_x.iter().zip(data_y.iter());

    let mut growing_products = 0.0;
    for (datum_x, datum_y) in zipped {
        growing_products += (f64::convert(*datum_x) - mean_x) * (f64::convert(*datum_y) - mean_y);
    }

    growing_products / (data_x.len() as f64 - 1.0)
}

pub fn pearson_r_method_1<T: Copy>(data_x: &Vec<T>, data_y: &Vec<T>, pop: Option<bool>) -> f64
where
    f64: Convert<T>,
{
    covariance(data_x, data_y) / (standard_deviation(Some(data_x), None, pop) * standard_deviation(Some(data_y), None, pop))
}

pub fn pearson_r_method_2<T: Copy>(data_x: &Vec<T>, data_y: &Vec<T>, pop: Option<bool>) -> f64
where
    f64: Convert<T>,
{
    let mean_x = mean(data_x);
    let mean_y = mean(data_y);
    let sd_x = standard_deviation(Some(data_x), None, pop);
    let sd_y = standard_deviation(Some(data_y), None, pop);

    let zipped = data_x.iter().zip(data_y.iter());

    let mut growing_products = 0.0;
    for (datum_x, datum_y) in zipped {
        growing_products += ((f64::convert(*datum_x) - mean_x) / sd_x) * ((f64::convert(*datum_y) - mean_y) / sd_y);
    }
    growing_products / (data_x.len() as f64 - 1.0)
}

pub fn t_statistic_from_r<T: Copy>(r: f64, n: T) -> f64
where
    f64: Convert<T>,
{
    r * f64::sqrt(f64::convert(n) - 2.0) / f64::sqrt(1.0 - f64::powi(r, 2))
}

pub fn pearson_r_from_t_statistic<T: Copy>(t: f64, n: T) -> f64
where
    f64: Convert<T>,
{
    let r = t / (f64::sqrt(f64::convert(n) - 2.0 + f64::powi(t, 2)));
    if t < 0.0 { -r } else { r }
}

pub fn covariance_from_r<T: Copy>(r: f64, data_xy: Option<(&Vec<T>, &Vec<T>)>, sd_xy: Option<(f64, f64)>) -> f64
where
    f64: Convert<T>,
{
    match data_xy {
        Some((data_x, data_y)) => {
            let sd_x = standard_deviation(Some(data_x), None, None);
            let sd_y = standard_deviation(Some(data_y), None, None);
            r * sd_x * sd_y
        }
        None => {
            match sd_xy {
                Some((sd_x, sd_y)) => {
                    r * sd_x * sd_y
                }
                None => {
                    panic!("Values provided to function contain None Types!")
                }
            }
        }
    }
}
