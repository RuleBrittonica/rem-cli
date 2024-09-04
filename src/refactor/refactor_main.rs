

/// Calls out to rem-controller, then rem-borrower, then rem-repairer to fix up
/// the extracted method.
///
/// Sequentially calls the nlc, borrower and repairer defined in this module as
/// well. Will iterate through each repair method until one works.
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
    file_path:       &str,
    new_file_path:   &str,
    callee_fn_name:  &str,
    caller_fn_name:  &str,

) -> bool {
    // Change this to return a result.
    todo!()
}

pub fn extract_function_generic(
    file_path:       &str,
    new_file_path:   &str,
    callee_fn_name:  &str,
    caller_fn_name:  &str,
) -> bool {
    todo!()
}

pub fn extract_function_async(
    file_path:       &str,
    new_file_path:   &str,
    callee_fn_name:  &str,
    caller_fn_name:  &str,
) -> bool {
    todo!()
}