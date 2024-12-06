use crate::data_types::data_array::{CategoricalDataArray, ContinuousDataArray};
// use crate::data_types::multiple_regression::MultipleRegression;
use crate::data_types::statistics::{IndependentGroupsT, PairedSamplesT, SingleSampleT, ANOVA};
use crate::data_types::csv::import_csv_data;
// use crate::functions::stats_math::{covariance, pearson_r_method_1, t_statistic_from_r};
use anyhow::{Error, Result};
// use charming::series::{Line, Scatter};
use log::info;
use std::path::Path;
// pub fn run_menudo_test() -> Result<(), Error> {
//     let menudo_file_path = Path::new("./csv-files/menudo.csv");
//     let menudo_csv_data = import_csv_data(menudo_file_path, None, None)?;
//
//     let tenure_data_array = menudo_csv_data.get_column_as_data_array::<i32>(
//         String::from("Member Tenures"),
//         5,
//         false,
//         true,
//     )?;
//     let ending_age_data_array = menudo_csv_data.get_column_as_data_array::<i32>(
//         String::from("Member Ending Ages"),
//         4,
//         false,
//         true,
//     )?;
//
//     let tenure_vs_ending_age_relationship = DataRelationship::new(
//         String::from("Member Tenure vs Ending Ages"),
//         &tenure_data_array,
//         &ending_age_data_array,
//     )?;
//
//     tenure_data_array.print_data();
//     ending_age_data_array.print_data();
//     tenure_vs_ending_age_relationship.print_relationship();
//     Ok(())
// }
//
// pub fn run_months_ice_cream() -> Result<(), Error> {
//     let dating_ice_cream_file_path = Path::new("./csv-files/dating-ice-cream.csv");
//     let dating_ice_cream_csv_data = import_csv_data(dating_ice_cream_file_path, None, None)?;
//
//     let relationship_months_data_array = dating_ice_cream_csv_data
//         .get_column_as_data_array::<i32>(
//             String::from("Length of Relationship in Months"),
//             1,
//             false,
//             false,
//         )?;
//     let pints_of_ice_cream_data_array = dating_ice_cream_csv_data.get_column_as_data_array::<i32>(
//         String::from("Pints of Ice Cream Eaten"),
//         2,
//         false,
//         false,
//     )?;
//
//     let relationship_vs_ice_cream_relationship = DataRelationship::new(
//         String::from("Length of Relationship vs Pints of Ice Cream Eaten"),
//         &relationship_months_data_array,
//         &pints_of_ice_cream_data_array,
//     )?;
//
//     relationship_months_data_array.print_data();
//     pints_of_ice_cream_data_array.print_data();
//     relationship_vs_ice_cream_relationship.print_relationship();
//     Ok(())
// }
//
// pub fn run_coffee_sleep_donuts() -> Result<(), Error> {
//     let coffee_sleep_donuts = import_csv_data(
//         "./csv-files/coffee-area-sleep-donuts.csv".as_ref(),
//         None,
//         None,
//     );
//
//     match coffee_sleep_donuts {
//         Ok(mut data) => {
//             info!("Imported CSV successfully!");
//             // let coffee = data.get_col::<i32>(1, None);
//             let sleep = data.get_column::<i32>(3, None)?;
//             let donuts = data.get_column::<i32>(4, None)?;
//
//             // let coffee_data_array = DataArray::new(coffee, None);
//             let sleep_data_array = DataArray::new(String::from("Hours of Sleep"), sleep, None)?;
//             let donuts_data_array = DataArray::new(String::from("Donuts Eaten"), donuts, None)?;
//             let pearson_r =
//                 pearson_r_method_1(&sleep_data_array.data, &donuts_data_array.data, None)?;
//
//             let zipped = sleep_data_array
//                 .data
//                 .iter()
//                 .zip(donuts_data_array.data.iter());
//
//             let mut growing_products = 0.0;
//             for (datum_x, datum_y) in zipped {
//                 growing_products += ((f64::convert(*datum_x) - sleep_data_array.mean)
//                     / sleep_data_array.standard_deviation)
//                     * ((f64::convert(*datum_y) - donuts_data_array.mean)
//                         / donuts_data_array.standard_deviation);
//             }
//
//             // info!("====COFFEE====");
//             // info!("{:#?}\n", coffee_data_array);
//             info!("====SLEEP=====");
//             info!("{:#?}\n", sleep_data_array);
//             info!("====DONUTS====");
//             info!("{:#?}\n", donuts_data_array);
//             info!(
//                 "Covariance: {}",
//                 covariance(&sleep_data_array.data, &donuts_data_array.data)?
//             );
//             info!(
//                 "Product of SDs: {}",
//                 sleep_data_array.standard_deviation * donuts_data_array.standard_deviation
//             );
//             info!("Pearson r: {}", pearson_r);
//             info!(
//                 "t value: {}",
//                 t_statistic_from_r(pearson_r, sleep_data_array.data.len())?
//             );
//             info!("products of z scores: {}", growing_products);
//             Ok(())
//         }
//         Err(_) => {
//             error!("Imported CSV failed!");
//             process::exit(1);
//         }
//     }
// }
//
// pub fn run_spotify_streaming() -> Result<(), Error> {
//     let spotify_file_path = Path::new("./csv-files/spotify-streaming.csv");
//     let spotify_csv_data = import_csv_data(spotify_file_path, None, None)?;
//
//     let spotify_total_playlists_data_array = spotify_csv_data.get_column_as_data_array::<i64>(
//         String::from("Total Playlists Count"),
//         6,
//         false,
//         false,
//     )?;
//     spotify_total_playlists_data_array.print_data();
//
//     let spotify_total_streams_data_array = spotify_csv_data.get_column_as_data_array::<i64>(
//         String::from("Total Stream Count"),
//         8,
//         false,
//         false,
//     )?;
//     spotify_total_streams_data_array.print_data();
//
//     let spotify_playlists_vs_streams_relationship = DataRelationship::new(
//         String::from("Spotify Playlists vs Stream Count"),
//         &spotify_total_playlists_data_array,
//         &spotify_total_streams_data_array,
//     )?;
//
//     spotify_playlists_vs_streams_relationship.print_relationship();
//
//     // Scatter::graph(&spotify_playlists_vs_streams_relationship)?;
//     // Line::graph(&spotify_playlists_vs_streams_relationship)?;
//
//     Ok(())
// }
//
// pub fn run_stress_levels() -> Result<(), Error> {
//     let candy_stress_vacation_file_path = Path::new("./csv-files/candy-stress-vacation.csv");
//     let candy_stress_vacation_csv_data =
//         import_csv_data(candy_stress_vacation_file_path, None, None)?;
//
//     let candy_bars_eaten_data_array = candy_stress_vacation_csv_data
//         .get_column_as_data_array::<i32>(String::from("Total Candy Bars Eaten"), 1, false, false)?;
//     let stress_level_data_array = candy_stress_vacation_csv_data.get_column_as_data_array::<i32>(
//         String::from("Stress Level"),
//         2,
//         false,
//         false,
//     )?;
//     let days_since_vacation_data_array = candy_stress_vacation_csv_data
//         .get_column_as_data_array::<i32>(
//             String::from("Days Since Last Vacation"),
//             3,
//             false,
//             false,
//         )?;
//     let weeks_since_vacation_data_array = candy_stress_vacation_csv_data
//         .get_column_as_data_array::<i32>(
//             String::from("Weeks Since Last Vacation"),
//             4,
//             false,
//             false,
//         )?;
//     let fortnights_since_vacation_data_array = candy_stress_vacation_csv_data
//         .get_column_as_data_array::<f32>(
//             String::from("Fortnights Since Last Vacation"),
//             5,
//             false,
//             false,
//         )?;
//
//     let candy_bars_vs_stress_relationship = DataRelationship::new(
//         String::from("Candy Bars vs Stress Relationship"),
//         &candy_bars_eaten_data_array,
//         &stress_level_data_array,
//     )?;
//
//     let stress_vs_candy_bars_relationship = DataRelationship::new(
//         String::from("Stress vs Candy Bars Relationship"),
//         &stress_level_data_array,
//         &candy_bars_eaten_data_array,
//     )?;
//
//     let weeks_vs_stress_relationship = DataRelationship::new(
//         String::from("Weeks Since Last Vacation vs Stress Relationship"),
//         &weeks_since_vacation_data_array,
//         &stress_level_data_array,
//     )?;
//
//     candy_bars_eaten_data_array.print_data();
//     stress_level_data_array.print_data();
//     days_since_vacation_data_array.print_data();
//     weeks_since_vacation_data_array.print_data();
//     fortnights_since_vacation_data_array.print_data();
//     candy_bars_vs_stress_relationship.print_relationship();
//     stress_vs_candy_bars_relationship.print_relationship();
//     weeks_vs_stress_relationship.print_relationship();
//
//     info!(
//         "Stress Level after 8 Weeks Without Vacation: {}",
//         weeks_vs_stress_relationship.get_y_hat(8.0)
//     );
//     info!(
//         "Candy Bars eaten with a Stress Level of 12: {}",
//         stress_vs_candy_bars_relationship.get_y_hat(12.0)
//     );
//     Ok(())
// }
//
// pub fn run_student_boredom() -> Result<(), Error> {
//     let student_boredom_file_path = Path::new("./csv-files/student-boredom.csv");
//     let student_boredom_csv_data = import_csv_data(student_boredom_file_path, None, None)?;
//
//     let minutes_backpack_data_array = student_boredom_csv_data.get_column_as_data_array::<i32>(
//         String::from("Minutes Wearing Backpack"),
//         1,
//         false,
//         false,
//     )?;
//     let lectures_attended_data_array = student_boredom_csv_data.get_column_as_data_array::<i32>(
//         String::from("Lectures Attended"),
//         2,
//         false,
//         false,
//     )?;
//
//     let student_boredom_data_array = student_boredom_csv_data.get_column_as_data_array::<i32>(
//         String::from("Student Boredom Level"),
//         3,
//         false,
//         false,
//     )?;
//
//     let lectures_boredom_relationship = DataRelationship::new(
//         String::from("Lectures Attended vs Boredom Relationship"),
//         &lectures_attended_data_array,
//         &student_boredom_data_array,
//     )?;
//
//     let backpack_boredom_relationship = DataRelationship::new(
//         String::from("Backpack Minutes vs Boredom Relationship"),
//         &minutes_backpack_data_array,
//         &student_boredom_data_array,
//     )?;
//
//     let backpack_lectures_relationship = DataRelationship::new(
//         String::from("Backpack vs Lectures Attended Relationship"),
//         &minutes_backpack_data_array,
//         &lectures_attended_data_array,
//     )?;
//
//     minutes_backpack_data_array.print_data();
//     lectures_attended_data_array.print_data();
//     student_boredom_data_array.print_data();
//     lectures_boredom_relationship.print_relationship();
//     backpack_boredom_relationship.print_relationship();
//     backpack_lectures_relationship.print_relationship();
//
//     // info!("x normal dist: {}", minutes_backpack_data_array.get_probability_density(50.0)?);
//
//     Ok(())
// }
//
// pub fn run_soda_bathroom() -> Result<(), Error> {
//     let lab_7_file_path = Path::new("./csv-files/labs/07_2024-10-15.csv");
//     let soda_bathroom_csv_data = import_csv_data(lab_7_file_path, None, None)?;
//
//     let ounce_of_soda_data_array = soda_bathroom_csv_data.get_column_as_data_array::<i32>(
//         String::from("Ounces of Soda Drunk"),
//         1,
//         false,
//         false,
//     )?;
//     let trips_to_bathroom_data_array = soda_bathroom_csv_data.get_column_as_data_array::<i32>(
//         String::from("Trips to Bathroom"),
//         2,
//         false,
//         false,
//     )?;
//     let soda_bathroom_relationship = DataRelationship::new(
//         String::from("Ounces of Soda Pop vs Trips to Bathroom"),
//         &ounce_of_soda_data_array,
//         &trips_to_bathroom_data_array,
//     )?;
//
//     soda_bathroom_relationship.print_relationship();
//     info!(
//         "How many times they go to the bathroom after 70 oz of soda: {}",
//         soda_bathroom_relationship.get_y_hat(70.0)
//     );
//     Ok(())
// }
//
// pub fn run_rent_cockroaches() -> Result<(), Error> {
//     let rent_cockroaches_file_path = Path::new("./csv-files/rent-cockroaches.csv");
//     let rent_cockroaches_csv_data = import_csv_data(rent_cockroaches_file_path, None, None)?;
//
//     let rent_data_array = rent_cockroaches_csv_data.get_column_as_data_array::<i32>(
//         String::from("Rent"),
//         1,
//         false,
//         false,
//     )?;
//     let cockroaches_data_array = rent_cockroaches_csv_data.get_column_as_data_array::<i32>(
//         String::from("Cockroaches in Apartment"),
//         2,
//         false,
//         false,
//     )?;
//     let rent_cockroaches_relationship = DataRelationship::new(
//         String::from("Rent vs Cockroaches in Apartment"),
//         &rent_data_array,
//         &cockroaches_data_array,
//     )?;
//
//     rent_cockroaches_relationship.print_relationship();
//     info!(
//         "Number of cockroaches at $500 rent: {}",
//         rent_cockroaches_relationship.get_y_hat(500.0)
//     );
//     Ok(())
// }
//
// pub fn run_caffeine_sleep() -> Result<(), Error> {
//     let caffeine_sleep_path = Path::new("./csv-files/caffeine-sleep.csv");
//     let caffeine_sleep_csv_data = import_csv_data(caffeine_sleep_path, None, None)?;
//
//     let caffeine_data_array = caffeine_sleep_csv_data.get_column_as_data_array::<i32>(
//         String::from("Ounces of Caffeine Imbibed"),
//         1,
//         false,
//         false,
//     )?;
//     let sleep_data_array = caffeine_sleep_csv_data.get_column_as_data_array::<i32>(
//         String::from("Hours of Sleep"),
//         2,
//         false,
//         false,
//     )?;
//     let sleep_vs_caffeine_relationship = DataRelationship::new(
//         String::from("Hours of Sleep vs Ounce of Caffeine"),
//         &sleep_data_array,
//         &caffeine_data_array,
//     )?;
//
//     sleep_vs_caffeine_relationship.print_relationship();
//     info!(
//         "Caffeine consumed if 4 hours slept: {}",
//         sleep_vs_caffeine_relationship.get_y_hat(4.0)
//     );
//     Ok(())
// }
//
// pub fn run_halloween_candy() -> Result<(), Error> {
//     let halloween_candy_file_path = Path::new("./csv-files/halloween-candy.csv");
//     let halloween_candy_csv_data = import_csv_data(halloween_candy_file_path, None, None)?;
//
//     let age_data_array = halloween_candy_csv_data.get_column_as_data_array::<i32>(
//         String::from("Age"),
//         1,
//         false,
//         false,
//     )?;
//     let cuteness_data_array = halloween_candy_csv_data.get_column_as_data_array::<i32>(
//         String::from("Cuteness"),
//         3,
//         false,
//         false,
//     )?;
//     let income_data_array = halloween_candy_csv_data.get_column_as_data_array::<i32>(
//         String::from("Income"),
//         4,
//         false,
//         false,
//     )?;
//     let houses_visited_data_array = halloween_candy_csv_data.get_column_as_data_array::<i32>(
//         String::from("Houses Visited"),
//         5,
//         false,
//         false,
//     )?;
//     let candy_received_data_array = halloween_candy_csv_data.get_column_as_data_array::<i32>(
//         String::from("Candy Received"),
//         6,
//         false,
//         false,
//     )?;
//
//     let income_vs_candy_received_relationship = DataRelationship::new(
//         String::from("Income vs Candy Received"),
//         &income_data_array,
//         &candy_received_data_array,
//     )?;
//     let houses_visited_vs_candy_relationship = DataRelationship::new(
//         String::from("Houses Visited vs Candy Received"),
//         &houses_visited_data_array,
//         &candy_received_data_array,
//     )?;
//     let cuteness_rating_vs_candy_relationship = DataRelationship::new(
//         String::from("Cuteness Rating vs Candy Received"),
//         &cuteness_data_array,
//         &candy_received_data_array,
//     )?;
//     let age_vs_cuteness_rating_relationship = DataRelationship::new(
//         String::from("Age vs Cuteness Rating"),
//         &age_data_array,
//         &cuteness_data_array,
//     )?;
//
//     income_vs_candy_received_relationship.print_relationship();
//     houses_visited_vs_candy_relationship.print_relationship();
//     cuteness_rating_vs_candy_relationship.print_relationship();
//     age_vs_cuteness_rating_relationship.print_relationship();
//
//     let multi_regression = MultipleRegression::new(
//         String::from("Test Multi Regression"),
//         &candy_received_data_array,
//         vec![&cuteness_data_array, &income_data_array, &age_data_array],
//     )?;
//
//     multi_regression.print_multiple_regression();
//
//     // info!("Cute at 1.5 yrs = {}", age_vs_cuteness_rating_relationship.get_y_hat(2.3));
//     // info!("Candy from $billion neighborhood {}", income_vs_candy_received_relationship.get_y_hat(1000000000.00));
//
//     Ok(())
// }
//
// pub fn run_exam_2() -> Result<(), Error> {
//     let exam_2_data_path = Path::new("./csv-files/exam_2_data.csv");
//     let exam_2_data = import_csv_data(exam_2_data_path, None, None)?;
//
//     let siblings_data_array =
//         exam_2_data.get_column_as_data_array::<i32>(String::from("Siblings"), 2, false, false)?;
//     let cereal_data_array = exam_2_data.get_column_as_data_array::<i32>(
//         String::from("Bowls of Cereal"),
//         3,
//         false,
//         false,
//     )?;
//     let hours_homework_data_array = exam_2_data.get_column_as_data_array::<i32>(
//         String::from("Hours of Homework"),
//         4,
//         false,
//         false,
//     )?;
//
//     siblings_data_array.print_data();
//     cereal_data_array.print_data();
//     hours_homework_data_array.print_data();
//
//     let hours_of_homework_siblings_relationship = DataRelationship::new(
//         String::from("Hours of Homework vs Siblings"),
//         &hours_homework_data_array,
//         &siblings_data_array,
//     )?;
//
//     hours_of_homework_siblings_relationship.print_relationship();
//
//     Ok(())
// }
//
// pub fn run_exam_2_followup() -> Result<(), Error> {
//     let exam_2_followup_data_path = Path::new("./csv-files/exam_2_followup_data.csv");
//     let exam_2_followup_data = import_csv_data(exam_2_followup_data_path, None, None)?;
//
//     let siblings_data_array = exam_2_followup_data.get_column_as_data_array::<i32>(
//         String::from("Siblings"),
//         1,
//         false,
//         false,
//     )?;
//     let cereal_data_array = exam_2_followup_data.get_column_as_data_array::<i32>(
//         String::from("Bowls of Cereal"),
//         2,
//         false,
//         false,
//     )?;
//     let hours_homework_data_array = exam_2_followup_data.get_column_as_data_array::<i32>(
//         String::from("Hours of Homework"),
//         3,
//         false,
//         false,
//     )?;
//
//     siblings_data_array.print_data();
//     // cereal_data_array.print_data();
//     hours_homework_data_array.print_data();
//
//     let siblings_hours_of_homework_relationship = DataRelationship::new(
//         String::from("Siblings vs Hours of Homework"),
//         &siblings_data_array,
//         &hours_homework_data_array,
//     )?;
//     // let hours_of_homework_cereal_relationship = SimpleLinearRegression::new(String::from("Hours of Homework vs Cereal"),
//     //                                                                         &hours_homework_data_array,
//     //                                                                         &cereal_data_array)?;
//
//     siblings_hours_of_homework_relationship.print_relationship();
//     // hours_of_homework_cereal_relationship.print_relationship();
//
//     // let siblings_hours_v_cereal = MultipleRegression::new(String::from("No. of Siblings and Hours of Homework v Cereal"),
//     //                                                       &cereal_data_array,
//     //                                                       vec![&siblings_data_array, &hours_homework_data_array])?;
//     //
//     // siblings_hours_v_cereal.print_multiple_regression();
//
//     // info!("0 hrs of homework = {} bowls of cereal", hours_of_homework_cereal_relationship.get_y_hat(0.0));
//     // info!("2 hrs of homework = {} bowls of cereal", hours_of_homework_cereal_relationship.get_y_hat(2.0));
//
//     // graph_test_simple_linear_regression(String::from("Siblings vs Hours of Homework"), &siblings_hours_of_homework_relationship)?;
//
//     Ok(())
// }
//
// pub fn run_superheroes() -> Result<(), Error> {
//     let superheroes_data_path = Path::new("../../csv-files/superheroes.csv");
//     let superheroes_csv_data = import_csv_data(superheroes_data_path, None, None)?;
//
//     // continuous
//     let nemeses_data_array = superheroes_csv_data.get_column_as_data_array::<i32>(
//         String::from("Nemeses"),
//         3,
//         false,
//         false,
//     )?;
//     // let sleep_before_data_array = get_column_as_data_array::<i32>(
//     //     &superheroes_csv_data,
//     //     String::from("Sleep Before"),
//     //     4,
//     //     false,
//     //     false,
//     // )?;
//     // let sleep_after_data_array = get_column_as_data_array::<i32>(
//     //     &superheroes_csv_data,
//     //     String::from("Sleep After"),
//     //     5,
//     //     false,
//     //     false,
//     // )?;
//     // let damage_before_data_array = get_column_as_data_array::<i32>(
//     //     &superheroes_csv_data,
//     //     String::from("Damage Before"),
//     //     6,
//     //     false,
//     //     false,
//     // )?;
//     // let damage_after_data_array = get_column_as_data_array::<i32>(
//     //     &superheroes_csv_data,
//     //     String::from("Damage After"),
//     //     7,
//     //     false,
//     //     false,
//     // )?;
//     let baby_powder_data_array = superheroes_csv_data.get_column_as_data_array::<i32>(
//         String::from("Baby Powder"),
//         8,
//         false,
//         false,
//     )?;
//
//     nemeses_data_array.print_data();
//     // sleep_before_data_array.print_data();
//     // sleep_after_data_array.print_data();
//     // damage_before_data_array.print_data();
//     // damage_after_data_array.print_data();
//     baby_powder_data_array.print_data();
//
//     // let nemeses_vs_damage_after = SimpleLinearRegression::new(
//     //     String::from("Nemeses vs Damage After"),
//     //     &nemeses_data_array,
//     //     &damage_after_data_array,
//     // )?;
//     let nemeses_vs_baby_powder = DataRelationship::new(
//         String::from("Nemeses vs Baby Powder"),
//         &nemeses_data_array,
//         &baby_powder_data_array,
//     )?;
//
//     // let nemeses_vs_multiple = MultipleRegression::new(String::from("Nemeses vs Multiple"),
//     //                                                   &nemeses_data_array,
//     //                                                   vec![
//     //                                                       &sleep_before_data_array,
//     //                                                       &sleep_after_data_array,
//     //                                                       &damage_before_data_array,
//     //                                                       &damage_after_data_array,
//     //                                                       &baby_powder_data_array])?;
//
//     // nemeses_vs_damage_after.print_relationship();
//     nemeses_vs_baby_powder.print_relationship();
//     // nemeses_vs_multiple.print_multiple_regression();
//
//     // graph_test(String::from("Nemeses vs Baby Powder"), nemeses_vs_baby_powder)?;
//
//     // info!("10 Nemeses = ${} in damage", nemeses_vs_damage_after.get_y_hat(10.0));
//     // info!("10 Nemeses = {} Bottles of Baby Powder", nemeses_vs_baby_powder.get_y_hat(10.0));
//
//     Ok(())
// }
//
// pub fn run_tinder_test() -> Result<(), Error> {
//     let tinder_data_path = Path::new("./csv-files/tinder.csv");
//     let tinder_csv_data = import_csv_data(tinder_data_path, None, None)?;
//
//     let tinder_data_array =
//         tinder_csv_data.get_column_as_data_array::<i32>(String::from("Tinder"), 1, false, false)?;
//
//     tinder_data_array.print_data();
//     info!(
//         "t-statistic, with mu of 35: {}",
//         tinder_data_array.get_single_t(35.0)?
//     );
//
//     Ok(())
// }
// pub fn run_homework_test() -> Result<(), Error> {
//     let homework_data_path = Path::new("./csv-files/homework.csv");
//     let homework_csv_data = import_csv_data(homework_data_path, None, None)?;
//
//     let money_before_data_array = homework_csv_data.get_column_as_data_array::<i32>(
//         String::from("Money Before"),
//         1,
//         false,
//         false,
//     )?;
//     let money_after_data_array = homework_csv_data.get_column_as_data_array::<i32>(
//         String::from("Money After"),
//         2,
//         false,
//         false,
//     )?;
//     let free_before_data_array = homework_csv_data.get_column_as_data_array::<i32>(
//         String::from("Free Time Before"),
//         3,
//         false,
//         false,
//     )?;
//     let free_after_data_array = homework_csv_data.get_column_as_data_array::<i32>(
//         String::from("Free Time After"),
//         4,
//         false,
//         false,
//     )?;
//
//     Ok(())
// }
//
// pub fn run_gpa_test() -> Result<(), Error> {
//     let gpa_data_path = Path::new("./csv-files/gpa.csv");
//     let gpa_csv_data = import_csv_data(gpa_data_path, None, None)?;
//
//     let gpa_1_data_array =
//         gpa_csv_data.get_column_as_data_array::<f64>(String::from("GPA 1"), 1, false, false)?;
//     let gpa_2_data_array =
//         gpa_csv_data.get_column_as_data_array::<f64>(String::from("GPA 2"), 2, false, false)?;
//
//     gpa_1_data_array.print_data();
//     gpa_2_data_array.print_data();
//
//     let gpa_relationship = DataRelationship::new(
//         String::from("GPA Relationship"),
//         &gpa_1_data_array,
//         &gpa_2_data_array,
//     )?;
//
//     gpa_relationship.print_relationship();
//
//     Ok(())
// }

pub fn run_glasses_occupation_likes_test() -> Result<(), Error> {
    let glasses_occupation_likes_path = Path::new("./csv-files/glasses_occupation_likes.csv");
    let glasses_occupation_likes_csv_data =
        import_csv_data(glasses_occupation_likes_path, None, None)?;

    let sleep_vec = &glasses_occupation_likes_csv_data.get_column::<f64>(4, Some(false))?;
    let employment_vec = &glasses_occupation_likes_csv_data.get_column::<String>(2, Some(false))?;

    let sleep_data_array =
        ContinuousDataArray::new(String::from("Sleep"), sleep_vec, 4, Some(false))?;

    let employment_data_array =
        CategoricalDataArray::new(String::from("Employment"), employment_vec, 2, Some(false))?;

    sleep_data_array.print();
    employment_data_array.print();

    let employment_sleep_independent_t = IndependentGroupsT::new(
        String::from("Employment vs Sleep"),
        String::from("Students get more sleep than those who are employed."),
        &employment_data_array,
        &sleep_data_array,
    )?;

    employment_sleep_independent_t.print();

    Ok(())
}

pub fn run_anova_sample_test() -> Result<(), Error> {
    let anova_sample_path = Path::new("./csv-files/anova_sample.csv");
    let anova_sample_csv_data = import_csv_data(anova_sample_path, None, None)?;

    let school_vec = &anova_sample_csv_data.get_column::<String>(1, Some(false))?;
    let gpa_vec = &anova_sample_csv_data.get_column::<f64>(4, Some(false))?;

    let school_data_array =
        CategoricalDataArray::new(String::from("School"), school_vec, 1, Some(false))?;
    let gpa_data_array = ContinuousDataArray::new(String::from("GPA"), gpa_vec, 4, Some(false))?;

    let school_vs_gpa_anova = ANOVA::new(
        String::from("School vs GPA"),
        String::from(
            "There is a difference in the means of GPAs between CU Boulder, CU Denver and CSU.",
        ),
        &school_data_array,
        &gpa_data_array,
    )?;

    school_vs_gpa_anova.print();

    Ok(())
}

pub fn run_exam_3_review_test() -> Result<(), Error> {
    let exam_3_path = Path::new("./csv-files/exam_3_review_data.csv");
    let exam_3_review_csv_data = import_csv_data(exam_3_path, None, None)?;

    let drinks_vec = &exam_3_review_csv_data.get_column::<String>(4, Some(false))?;
    let headphones_vec = &exam_3_review_csv_data.get_column::<f64>(7, Some(false))?;
    let sleep_nov_vec = &exam_3_review_csv_data.get_column::<f64>(3, Some(false))?;

    let drinks_data_array =
        CategoricalDataArray::new(String::from("Drinks"), drinks_vec, 4, Some(false))?;
    let headphones_data_array =
        ContinuousDataArray::new(String::from("Headphones"), headphones_vec, 7, Some(false))?;
    let november_sleep_data_array = ContinuousDataArray::new(
        String::from("Hours of Sleep in November"),
        sleep_nov_vec,
        3,
        Some(false),
    )?;

    let drinks_vs_headphones_anova = ANOVA::new(
        String::from("Drinks vs Headphones"),
        String::from("There is a difference in the means of headphones ownership based on drinks preference."),
        &drinks_data_array,
        &headphones_data_array
    )?;

    let drinks_vs_nov_sleep = ANOVA::new(
        String::from("Drinks vs Sleep"),
        String::from(
            "There is a difference in the means of sleep in november based on drinks preference.",
        ),
        &drinks_data_array,
        &november_sleep_data_array,
    )?;

    drinks_vs_headphones_anova.print();
    drinks_vs_nov_sleep.print();

    Ok(())
}

pub fn run_movie_data_test() -> Result<(), Error> {
    let movie_path = Path::new("./csv-files/movie_data.csv");
    let movie_data_csv_data = import_csv_data(movie_path, None, None)?;

    let year_vec = &movie_data_csv_data.get_column::<String>(8, Some(false))?;

    let runtime_hours_vec = &movie_data_csv_data.get_column::<i32>(9, Some(false))?;
    let runtime_minutes_vec = &movie_data_csv_data.get_column::<i32>(10, Some(false))?;
    let mut runtime_vec = Vec::with_capacity(year_vec.len());

    let zipped = runtime_hours_vec.iter().zip(runtime_minutes_vec.iter());
    for (hour, minute) in zipped {
        let total_minutes = ((hour * 60) + minute) as f64;
        runtime_vec.push(total_minutes);
    }

    let decade_data_array =
        CategoricalDataArray::new(String::from("Year"), &year_vec, 9, Some(false))?;
    let runtime_data_array =
        ContinuousDataArray::new(String::from("Runtime"), &runtime_vec, 0, Some(false))?;

    let year_runtime_independent_groups = ANOVA::new(
        String::from("Runtime vs Decade"),
        String::from("Decades have trends of popular lengths of films."),
        &decade_data_array,
        &runtime_data_array,
    )?;

    year_runtime_independent_groups.print();

    Ok(())
}
