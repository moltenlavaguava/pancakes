use futures::future;
use tokio::runtime::Runtime;

use crate::service::gui::run_gui;

pub fn run_program() {
    let runtime = Runtime::new().expect("Failed to create tokio runtime");
    let _guard = runtime.enter();

    // start gui
    run_gui().expect("Failed to start gui");

    // let _ = runtime.block_on(future::join_all(vec![]));
}
