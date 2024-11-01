mod data_types;
mod error_types;
mod functions;
mod logging;
mod tests;
mod graphing;

use std::error::Error;
use std::fmt::{Debug, Display};
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
use log::info;
// use crate::graphing::graph_test;
use crate::logging::{format_title, setup_logger};
use crate::tests::tests::*;


fn run_ratatui(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue();
            frame.render_widget(greeting, frame.area());
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

fn main() -> Result<()> {
    setup_logger().expect("Logging setup failed.");
    info!("{}", format_title(&*"Stisty"));
    info!("{}", format_title(&*""));

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
    run_exam_2_followup().expect("exam 2 followup failed");
    // run_superheroes().expect("superheroes failed");

    info!("{}", format_title(&*""));

    Ok(())
}
