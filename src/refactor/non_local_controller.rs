use rem_controller::non_local_controller;
use core::error;
use std::{
    time::{
        Duration,
        Instant,
    },
    path::Path,
    fs,
};
use log::{
    info,
    error,
};


pub fn non_local_controller(
    file_path: &str,
    new_file_path: &str,
    callee_fn_name: &str,
    caller_fn_name: &str,
) -> bool {
    // Backup the original file
    let backup: String = format!("/tmp/{}-rs-extract.bk", file_path);
    if let Err(e) = fs::copy(file_path, &backup) {
        error!("Failed to created backup: {:?}", e);
        return false;
    }

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
        info!("Bad exit value, restoring file");
        if let Err(e) = fs::copy(&backup, file_path) {
            error!("Failed to restore file: {:?}", e);
        }
    }

    success
}