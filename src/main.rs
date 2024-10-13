mod functions;
mod logging;
mod data_types;
mod error_types;

// use std::error::Error;
use std::fmt::Display;
use anyhow::Result;
// ratatui modules
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};

use std::io;
use std::iter::Sum;
use std::path::Path;
use functions::{stats_math, csv::import_csv_data};
use std::process;
use log::{error, info};
use crate::functions::stats_math::{covariance, deviation, pearson_r_from_t_statistic, pearson_r_method_1, t_statistic_from_r, x_from_znpq, z_score_from_normal_approximation};
use crate::logging::setup_logger;
use crate::data_types::data_array::DataArray;
use crate::data_types::relationship::Relationship;
use crate::error_types::CSVError;
use crate::functions::convert::Convert;

// fn run_menudo_test() {
//     let menudo_data = import_csv_data("./csv-files/menudo.csv", None, None);
//     let mut unwrapped_menudo;
//     match menudo_data {
//         Ok(ok) => {
//             unwrapped_menudo = ok;
//             println!("Imported CSV successfully!");
//         }
//         Err(_) => {
//             println!("Imported CSV failed!");
//             process::exit(1);
//         }
//     }
//
//     let menudo_tenure = unwrapped_menudo.get_col::<i32>(6, None);
//     let menudo_end_age = unwrapped_menudo.get_col::<i32>(5, None);
//
//     println!("================================================");
//     println!("===================MENUDO TEST==================");
//     println!("Column Data: {:?}", menudo_tenure);
//     let ss = stats_math::sum_of_squares(&menudo_tenure);
//     let deviation = stats_math::deviation(menudo_tenure[13], &menudo_tenure);
//     let variance = stats_math::variance(&menudo_tenure, None);
//     let standard_deviation = stats_math::standard_deviation(Some(&menudo_tenure), None, Some(true));
//     let z_score = stats_math::z_score_from_deviation(deviation, &menudo_tenure, Some(true));
//     let raw_from_z = stats_math::raw_score_from_z_data(z_score, &menudo_tenure, Some(true));
//     let tenure_end_age_cov = stats_math::covariance(&menudo_tenure, &menudo_end_age);
//     let tenure_end_age_r = stats_math::pearson_r_method_1(&menudo_tenure, &menudo_end_age, Some(true));
//     println!("Tenure SS: {}", ss);
//     println!("#14 Tenure Deviation: {}", deviation);
//     println!("Tenure Variance: {}", variance);
//     println!("Tenure SD: {}", standard_deviation);
//     println!("#14 Tenure Z-Score: {}", z_score);
//     println!("#14 Raw Score: {}", raw_from_z);
//     println!("Tenure vs End Age Covariance: {}", tenure_end_age_cov);
//     println!("Tenure vs End Age r: {}", tenure_end_age_r);
// }

// fn run_stress_candy_test() {
//     let stress_candy_data = import_csv_data("./csv-files/pearson-r-data.csv", None, None);
//
//     let mut unwrapped_stress_candy;
//     match stress_candy_data {
//         Ok(ok) => {
//             unwrapped_stress_candy = ok;
//             println!("Imported CSV successfully!");
//         }
//         Err(_) => {
//             println!("Imported CSV failed!");
//             process::exit(1);
//         }
//     }
//
//     let candy = unwrapped_stress_candy.get_col::<i32>(2, Some(true));
//     let stress = unwrapped_stress_candy.get_col::<i32>(3, Some(true));
//     let weeks = unwrapped_stress_candy.get_col::<i32>(5, Some(true));
//
//     let candy_4_z_1 = stats_math::z_score(Some(candy[3]), None, Some(&candy), None, None, None);
//     // let candy_4_z_2 = stats_math::z_score(None, Some(deviation(candy[3], &candy)), Some(&candy), None, None, None);
//
//     let candy_sd = stats_math::standard_deviation(Some(&candy), None, Some(false));
//     let stress_sd = stats_math::standard_deviation(Some(&stress), None, Some(false));
//     let weeks_sd = stats_math::standard_deviation(Some(&weeks), None, Some(false));
//     let stress_candy_cov = stats_math::covariance(&stress, &candy);
//     let stress_candy_r_1 = stats_math::pearson_r_method_1(&stress, &candy, Some(false));
//     let stress_candy_r_2 = stats_math::pearson_r_method_1(&stress, &candy, Some(false));
//     let stress_candy_t = stats_math::t_statistic_from_r(stress_candy_r_1, candy.len() as f64);
//     let stress_candy_r_from_t = stats_math::pearson_r_from_t_statistic(stress_candy_t, candy.len() as f64);
//     let stress_candy_cov_from_r = stats_math::covariance_from_r(stress_candy_r_from_t, Some((&stress, &candy)), None);
//
//     info!("================================================");
//     info!("====CANDY, STRESS, WEEKS SINCE VACATION DATA====");
//     info!("Candy Column Data: {:?}", candy);
//     info!("Stress Column Data: {:?}", stress);
//     info!("Weeks Since Last Vacation Column Data: {:?}", weeks);
//     info!("Candy SD: {}", candy_sd);
//     info!("Stress SD: {}", stress_sd);
//     info!("Weeks SD: {}", weeks_sd);
//     info!("Stress vs Candy Covariance: {}", stress_candy_cov);
//     info!("Stress vs Candy r (method 1): {}", stress_candy_r_1);
//     info!("Stress vs Candy r (method 2): {}", stress_candy_r_2);
//     info!("Stress vs Candy t: {}", stress_candy_t);
//     info!("Stress vs Candy r (from t): {}", stress_candy_r_from_t);
//     info!("Stress vs Candy Covariance (from r): {}", stress_candy_cov_from_r);
//     info!("Candy #4: {}", candy[2]);
//     info!("Candy #4 Z-Score: {}", candy_4_z_1);
//     // info!("Candy #4 Z-Score: {}", candy_4_z_2);
// }

// fn run_months_ice_cream() {
//     let months_ice_cream = import_csv_data("./csv-files/dating-ice-cream.csv", None, None);
//
//     match months_ice_cream {
//         Ok(ref data) => {
//             info!("Imported CSV successfully!");
//             let months = data.get_col::<i32>(1, None);
//             let ice_cream = data.get_col::<i32>(2, None);
//
//             let months_data_array = DataArray::new(String::from("Total Months Relationship Lasted"), months, None);
//             let ice_cream_data_array = DataArray::new(String::from("Pints of Ice Cream Eaten After Break Up"), ice_cream, None);
//
//             months_data_array.print_data();
//             ice_cream_data_array.print_data();
//
//             let relationship = Relationship::new(String::from("Months vs Ice Cream"), &months_data_array, &ice_cream_data_array, None);
//             relationship.print_relationship();
//         }
//         Err(_) => {
//             error!("Imported CSV failed!");
//             process::exit(1);
//         }
//     }
// }

// fn run_coffee_sleep_donuts() {
//     let coffee_sleep_donuts = import_csv_data("./csv-files/coffee-area-sleep-donuts.csv", None, None);
//
//     match coffee_sleep_donuts {
//         Ok(mut data) => {
//             info!("Imported CSV successfully!");
//             // let coffee = data.get_col::<i32>(1, None);
//             let sleep = data.get_col::<i32>(3, None);
//             let donuts = data.get_col::<i32>(4, None);
//
//             // let coffee_data_array = DataArray::new(coffee, None);
//             let sleep_data_array = DataArray::new(String::from("Hours of Sleep"), sleep, None);
//             let donuts_data_array = DataArray::new(String::from("Donuts Eaten"), donuts, None);
//             let pearson_r = pearson_r_method_1(&sleep_data_array.data, &donuts_data_array.data, None);
//
//             let zipped = sleep_data_array.data.iter().zip(donuts_data_array.data.iter());
//
//             let mut growing_products = 0.0;
//             for (datum_x, datum_y) in zipped {
//                 growing_products += ((f64::convert(*datum_x) - sleep_data_array.mean.unwrap()) / sleep_data_array.standard_deviation.unwrap()) * ((f64::convert(*datum_y) - donuts_data_array.mean.unwrap()) / donuts_data_array.standard_deviation.unwrap());
//             }
//
//             // info!("====COFFEE====");
//             // info!("{:#?}\n", coffee_data_array);
//             info!("====SLEEP=====");
//             info!("{:#?}\n", sleep_data_array);
//             info!("====DONUTS====");
//             info!("{:#?}\n", donuts_data_array);
//             info!("Covariance: {}", covariance(&sleep_data_array.data, &donuts_data_array.data));
//             info!("Product of SDs: {}", sleep_data_array.standard_deviation.unwrap() * donuts_data_array.standard_deviation.unwrap());
//             info!("Pearson r: {}", pearson_r);
//             info!("t value: {}", t_statistic_from_r(pearson_r, sleep_data_array.data.len()));
//             info!("products of z scores: {}", growing_products);
//         }
//         Err(_) => {
//             error!("Imported CSV failed!");
//             process::exit(1);
//         }
//     }
// }

fn run_spotify_streaming() -> Result<()> {
    let spotify_csv = import_csv_data(Path::new("./csv-files/spotify-streaming.csv"), None, None);
    match spotify_csv {
        Ok(ref data) => {
            let total_playlists_count = data.get_col::<i64>(6, None).map_err(|e| From::from(e));
            let total_streams_count = data.get_col::<i64>(8, None).map_err(|e| From::from(e));

            let playlists_data_array = DataArray::new(String::from("Total Playlists Count"), total_playlists_count?, None);
            let streams_data_array = DataArray::new(String::from("Total Streams Count"), total_streams_count?, None);
            let playlists_streams_relationship = Relationship::new(String::from("Total Playlist Count vs Total Stream Count"), &playlists_data_array, &streams_data_array, None);

            playlists_data_array.print_data();
            streams_data_array.print_data();
            playlists_streams_relationship.print_relationship();
            Ok(())
        }
        Err(err) => {
            error!("{}", err);
            Err(err)?
        }
    }
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue();
            frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

fn main() -> Result<()> {
    setup_logger().expect("Logging setup failed.");
    info!("==============================STISTY==============================");
    info!("==================================================================");

    //////// ratatui ////////
    // let mut terminal = ratatui::init();
    // terminal.clear()?;
    // let app_result = run(terminal);
    // ratatui::restore();
    // app_result

    // run_menudo_test();
    // run_stress_candy_test();
    // run_months_ice_cream();
    // run_coffee_sleep_donuts();
    run_spotify_streaming().unwrap_or_else(|e| {
        error!("{}", e);
        // panic!("{}", e)
    });

    Ok(())
}
