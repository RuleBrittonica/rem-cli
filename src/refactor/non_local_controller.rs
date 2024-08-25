use rem_controller::non_local_controller;
use std::{
    time::Instant,
    fs,
};
use log::{
    info,
    error,
};


pub fn non_local_controller(
    file_path:      &str,
    new_file_path:  &str,
    callee_fn_name: &str,
    caller_fn_name: &str,
    backup:         &str,
) -> bool {


    let begin: Instant = Instant::now();

    // Run the controller
    let success: bool = non_local_controller::make_controls(
        file_path,
        new_file_path,
        callee_fn_name,
        caller_fn_name
    );

    // Handle a controller failure
    if !success {
        info!("Bad exit value, restoring file in nlc");
        if let Err(e) = fs::copy(&backup, file_path) {
            error!("Failed to restore file in nlc: {:?}", e);
        }
    }

    let success_string: &str = if success { "was successful" } else { "failed" };

    info!("Controller {}, elapsed time in milliseconds: {:?}",
        success_string,
        begin.elapsed().as_millis()
    );

    success
}