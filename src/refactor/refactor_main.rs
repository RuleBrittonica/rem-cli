use log::{
    info,
    error
};

use crate::refactor::{
    borrow::borrow,
    non_local_controller::non_local_controller,
    repair_lifetime::repair_lifetime,
};

/// Calls out to rem-controller, then rem-borrower, then rem-repairer to fix up
/// the extracted method.
///
/// # Args
/// * `file_path` - The path to the original file. Must contain a new method with the signature extracted and the original code inside it
/// * `new_file_path` - The path to the new file. If it is the same as the original file, then we will overwrite the existing file.
/// * `calle_fn_name` - The function that has been extracted
/// * `caller_fn_name` - The function that contains the call to calle_fn_name
///
/// # Returns
/// * bool - True if extraction was successful.
pub fn extract_function(
    file_path:      &str,
    new_file_path:  &str,
    callee_fn_name: &str,
    caller_fn_name: &str,
    backup:         &str,
) -> bool {
    // Log successful dump
    info!("Dumped call types completed successfully");

    // Attempt to run the controller
    if !non_local_controller(file_path, new_file_path, callee_fn_name, caller_fn_name, backup) {
        error!("Controller NOT completed - Borrower and Repairer will not be executed");
        return false;
    }

    // Log successful controller
    info!("Controller completed successfully");

    // Attempt to run the borrower
    if !borrow(new_file_path, callee_fn_name, caller_fn_name, backup) {
        error!("Borrow NOT completed - Repairer will not be executed");
        return false;
    }

    // Log successful borrow
    info!("Borrow completed successfully");

    // Attempt to run the repairer
    if !repair_lifetime(new_file_path, callee_fn_name, caller_fn_name, backup) {
        error!("Repairer NOT completed");
        return false;
    }

    // Log successful repair of lifetimes
    info!("Repairer completed successfully");

    return true;
}

pub fn extract_function_generic(
    file_path:      &str,
    new_file_path:  &str,
    callee_fn_name: &str,
    caller_fn_name: &str,
    backup:         &str,
) -> bool {
    todo!()
}

pub fn extract_function_async(
    file_path:      &str,
    new_file_path:  &str,
    callee_fn_name: &str,
    caller_fn_name: &str,
    backup:         &str,
) -> bool {
    todo!()
}