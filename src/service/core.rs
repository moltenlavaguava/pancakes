use futures::future;
use tokio::{runtime::Runtime, sync::mpsc};
use tokio_util::sync::CancellationToken;

use crate::service::{
    file::FileService, gui::run_gui, logic::run_service, process::ProcessService,
    request::RequestService,
};

pub fn run_program() {
    let runtime = Runtime::new().expect("Failed to create tokio runtime");
    let end_token = CancellationToken::new();
    let _guard = runtime.enter();

    // Create channels for communication
    let (t_gui_event, r_gui_event) = mpsc::channel(100);
    let (t_process, r_process) = mpsc::channel(100);
    let (t_request, r_request) = mpsc::channel(100);
    let (t_file, r_file) = mpsc::channel(100);

    // Create services
    let process_service = ProcessService::new(t_gui_event.clone());
    let request_service = RequestService::new(t_gui_event.clone());
    let file_service = FileService::new();

    // Start services
    let process_handle = {
        let token = end_token.clone();
        tokio::spawn(async move { run_service(process_service, r_process, token).await })
    };
    let request_handle = {
        let token = end_token.clone();
        tokio::spawn(async move { run_service(request_service, r_request, token).await })
    };
    let file_handle = {
        let token = end_token.clone();
        tokio::spawn(async move { run_service(file_service, r_file, token).await })
    };

    // start gui
    run_gui(r_gui_event, t_request, t_file).expect("Failed to start gui");

    // send request to end program
    end_token.cancel();

    let _ = runtime.block_on(future::join_all(vec![
        process_handle,
        request_handle,
        file_handle,
    ]));
}
