mod functions;
mod logging;
mod data_types;
mod error_types;
mod tests;

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
use std::iter::Sum;
use std::path::Path;
use functions::{stats_math, csv::import_csv_data};
use std::process;
use std::str::FromStr;
use log::{error, info};
use crate::logging::setup_logger;
use crate::data_types::data_array::DataArray;
use crate::data_types::relationship::Relationship;
use crate::functions::convert::Convert;
use crate::functions::csv::CSVData;
use crate::tests::tests::{run_spotify_streaming, run_stress_levels};

fn get_data_stats<T>(csv_data: &CSVData, data_name: String, column: usize, one_based_index: bool, population: bool) -> Result<DataArray, CSVError<T>>
where
    T: FromStr + Clone + Copy + Debug + 'static,
    <T as FromStr>::Err: Error + Send + Sync + 'static,
    f64: Convert<T>,
{
    Ok(DataArray::new(data_name, csv_data.get_col::<T>(column, Some(one_based_index))
        .map_err(|error| <CSVError<T>>::from(error))?, Some(population)))
}

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
    info!("==============================STISTY==============================");
    info!("==================================================================");

    //////// ratatui ////////
    // let mut terminal = ratatui::init();
    // terminal.clear()?;
    // let app_result = run(terminal);
    // ratatui::restore();
    // app_result

    run_spotify_streaming().expect("Spotify test failed.");
    run_stress_levels().expect("Stress levels test failed.");


    info!("=================================================================");

    Ok(())
}
