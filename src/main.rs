use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};


pub mod app; // Code for the ratatui portion of the program.
pub mod tui;
pub mod config; // Code for the handwritten modules.
pub mod gtfs_rt;
pub mod gtfs_static;
pub mod import;
pub mod search;

use crate::app::{App, CurrentScreen, CurrentlyEditing};
use crate::tui::ui;


// -------- BEGIN PROGRAM CODE -------- //
