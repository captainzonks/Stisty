pub mod build;
mod core;
mod data_types;
mod functions;
mod tests;

use crate::core::arg_handler::{generate_cli, process_args, run_cli};
use crate::core::menu::main_menu;
use crate::tests::tests::*;
use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use core::logging::{format_title, setup_logger};
use log::info;
use std::io;
use std::path::PathBuf;
// ratatui modules
// use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
// use ratatui::{
//     buffer::Buffer,
//     layout::{Alignment, Rect},
//     style::Stylize,
//     symbols::border,
//     text::{Line, Text},
//     widgets::{
//         block::{Position, Title},
//         Block, Paragraph, Widget,
//     },
//     DefaultTerminal, Frame,
// };

// fn run_ratatui(mut terminal: DefaultTerminal) -> io::Result<()> {
//     loop {
//         terminal.draw(|frame| {
//             let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
//                 .white()
//                 .on_blue();
//             frame.render_widget(greeting, frame.area());
//         })?;
//
//         if let Event::Key(key) = event::read()? {
//             if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
//                 return Ok(());
//             }
//         }
//     }
// }

fn main() -> Result<()> {
    setup_logger().expect("Logging setup failed.");
    info!("{}", format_title(&*"Stisty"));
    info!("{}", format_title(&*""));

    run_cli()?;

    // process_args()?;

    // main_menu().expect("Menu failed!");

    //////// ratatui ////////
    // let mut terminal = ratatui::init();
    // terminal.clear()?;
    // let app_result = run(terminal);
    // ratatui::restore();
    // app_result

    // run_menudo_test().expect("Menudo test failed.");
    // run_months_ice_cream().expect("Months ice cream test failed.");
    // run_spotify_streaming().expect("Spotify test failed.");
    // run_stress_levels().expect("Stress levels test failed.");
    // run_student_boredom().expect("Student boredom test failed.");
    // run_soda_bathroom().expect("Lab 7 failed");
    // run_rent_cockroaches().expect("Rent Cockroaches test failed");
    // run_caffeine_sleep().expect("Sleep caffeine test failed");
    // run_halloween_candy().expect("Halloween candy test failed");
    // run_exam_2().expect("exam 2 failed");
    // run_exam_2_followup().expect("exam 2 followup failed");
    // run_superheroes().expect("superheroes failed");
    // run_tinder_test().expect("Tinder test failed");
    // run_gpa_test().expect("GPA test failed");
    // run_glasses_occupation_likes_test().expect("student eyes test failed");
    // run_anova_sample_test().expect("ANOVA sample test failed.");
    // run_exam_3_review_test().expect("Exam 3 review test failed.");
    // run_movie_data_test().expect("Movie Data test failed.");

    info!("{}", format_title(&*""));

    Ok(())
}
