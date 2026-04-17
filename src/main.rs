#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pancakes::service::{core::run_program, gui::logging};

fn main() {
    let log_rx = logging::init();

    run_program(log_rx);
}
