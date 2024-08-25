use rem_borrower::borrow;
use std::time::Instant;
use log::info;

/// Only takes a file path as the non_local_controller has already setup the
/// final output filepath.
pub fn borrow(
    file_path:      &str,
    callee_fn_name: &str,
    caller_fn_name: &str,
    backup:         &str,
) -> bool {
    let pre_extract_file_name: &str = backup;
    let dump_file_name: &str = "/tmp/method_call_mutability.txt";

    let begin: Instant = Instant::now();

    // Run the borrower
    let success: bool = borrow::make_borrows(
        file_path,
        file_path,
        dump_file_name,
        callee_fn_name,
        caller_fn_name,
        pre_extract_file_name
    );

    let success_string: &str = if success { "was successful " } else { "failed" };

    info!("Borrower {}, elapsed time in milliseconds: {:?}",
        success_string,
        begin.elapsed().as_millis()
    );

    success

}