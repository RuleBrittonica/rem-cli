use log::{
    info,
    error
};

use crate::refactor::{
    borrow::borrow,
    non_local_controller::non_local_controller,
    repair_lifetime::repair_lifetime,
};

use crate::utils::ProgramOptions;

/// Calls out to rem-controller, then rem-borrower, then rem-repairer to fix up
/// the extracted method.
///
/// # Args
/// * `file_path` - The path to the original file. Must contain a new method with the signature extracted and the original code inside it
/// * `new_file_path` - The path to the new file. If it is the same as the original file, then we will overwrite the existing file.
/// * `calle_fn_name` - The function that has been extracted
/// * `caller_fn_name` - The function that contains the call to calle_fn_name
/// * `opt` - The program option that has been set (which part of the program to run)
///
/// # Returns
/// * bool - True if extraction was successful.
pub fn extract_function(
    file_path:      &str,
    new_file_path:  &str,
    callee_fn_name: &str,
    caller_fn_name: &str,
    backup:         &str,
    options:        Vec<ProgramOptions>,
) -> bool {
    for opt in options {
        match opt {
            ProgramOptions::Controller => {
                if !non_local_controller(file_path, new_file_path, callee_fn_name, caller_fn_name) {
                    error!("Controller NOT completed - halting further execution");
                    return false;
                }
                info!("Controller completed successfully");
            }
            ProgramOptions::Borrower => {
                if !borrow(file_path, new_file_path, callee_fn_name, caller_fn_name, backup) {
                    error!("Borrow NOT completed - halting further execution");
                    return false;
                }
                info!("Borrow completed successfully");
            }
            ProgramOptions::Repairer => {
                if !repair_lifetime(file_path, new_file_path, callee_fn_name, caller_fn_name) {
                    error!("Repairer NOT completed - halting further execution");
                    return false;
                }
                info!("Repairer completed successfully");
            }
        }
    }

    return true;
}

pub fn extract_function_generic(
    file_path:      &str,
    new_file_path:  &str,
    callee_fn_name: &str,
    caller_fn_name: &str,
    backup:         &str,
    options:        Vec<ProgramOptions>,
) -> bool {
    todo!()
}

pub fn extract_function_async(
    file_path:      &str,
    new_file_path:  &str,
    callee_fn_name: &str,
    caller_fn_name: &str,
    backup:         &str,
    options:        Vec<ProgramOptions>,
) -> bool {
    todo!()
}