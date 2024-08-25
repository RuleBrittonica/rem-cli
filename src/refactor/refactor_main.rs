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
    file_path: &str,
    new_file_path: &str,
    calle_fn_name: &str,
    caller_fn_name: &str,
) -> bool {
    // Log successful dump
    info!("Dumped call types completed successfully");

    if non_local_controller(file_path, new_file_path, calle_fn_name, caller_fn_name) {
        // Log successful controller
        info!("Controller completed successfully");

        // Borrower only takes the new file path, NLC has done the handling and
        // writing to the new file already
        if borrow(new_file_path, calle_fn_name, caller_fn_name) {
            // Log successful borrow
            info!("Borrow completed succesfully");

            // Similarly Repairer only takes the new file path.
            if repair_lifetime(new_file_path, calle_fn_name, caller_fn_name) {
                // Log successful repair of lifetimes
                info!("Repairer completed successfully");
                true // All stages complete

            } else {
                // Log unsuccessful repair of lifetimes
                error!("Repairer NOT completed");
                false
            }
        } else {
            // Log unsuccessful borrow
            error!("Borrow NOT completed - Repairer will not be executed");
            false
        }
    } else {
        // Log unsuccessful controller
        error!("Controller NOT completed - Borrower and Repairer will not be executed");
        false
    }
}

pub fn extract_function_generic(
    file_path: &str,
    new_file_path: &str,
    calle_fn_name: &str,
    caller_fn_name: &str,
) -> bool {
    todo!()
}

pub fn extract_function_async(
    file_path: &str,
    new_file_path: &str,
    calle_fn_name: &str,
    caller_fn_name: &str,
) -> bool {
    todo!()
}