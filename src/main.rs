#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pancakes::service::core::run_program;

fn main() {
    run_program();
}
