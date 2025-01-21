// mod core;
// mod data_types;
// mod functions;
// mod tests;
//
// use crate::core::arg_handler::run_cli;
// use anyhow::Result;
// use core::logging::{format_title, setup_logger};
// use log::info;
// use std::io;
// use std::path::PathBuf;
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

use anyhow::Result;
use stisty_lib::core::arg_handler::run_cli;
use stisty_lib::core::logging::{format_title, setup_logger};

fn main() -> Result<()> {
    setup_logger().expect("Logging setup failed.");
    log::info!("{}", format_title(&*"Stisty"));
    log::info!("{}", format_title(&*""));

    run_cli()?;

    //////// ratatui ////////
    // let mut terminal = ratatui::init();
    // terminal.clear()?;
    // let app_result = run(terminal);
    // ratatui::restore();
    // app_result

    log::info!("{}", format_title(&*""));

    Ok(())
}
