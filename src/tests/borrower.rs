use rem_borrower::borrow;
use crate::tests::utils::compile_file;
use log::{
    error,
    info,
};

use colored::Colorize;
use std::{
    fs::{self, read_to_string},
    io,
    path::{
        Path,
        PathBuf,
    },
    time::SystemTime
};

use crate::tests::utils::{
    cleanup_new_files,
    list_files_in_dir,
};

use crate::refactor::throughput::Borrower;

pub fn test(path: PathBuf) -> Result<u8, io::Error> {

    let folder_path: String = match path.to_str() {
        Some(path_str) => path_str.to_string(),
        None => {
            error!("Failed to conver path to string {:?}", path);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Path"));
        }
    };

    if !path.is_dir() {
        error!("The path provided is not a directory: {}", folder_path);
        return Err(io::Error::new(io::ErrorKind::NotFound, "Path is not a directory"));
    }

    info!("Running tests from directory {}{}", folder_path, "borrower");

    // Capture initial state
    let current_dir: &Path = Path::new("./");
    let initial_files: Vec<String> = list_files_in_dir(current_dir)?;

    let mut total_failed_tests: u8 = 0;

    for file in fs::read_dir(format!("{}/borrower/input", folder_path)).unwrap() {
        let file = file?;
        let test_name = file.file_name().to_owned();
        let file_name = format!("{}/borrower/input/{}", folder_path, test_name.to_str().unwrap());
        let new_file_name = format!("{}/borrower/output/{}", folder_path, test_name.to_str().unwrap());
        let mut_method_call_expr_file =
            format!("{}/borrower/method_call_mut/{}", folder_path, test_name.to_str().unwrap());
        let pre_extract_file_name = format!("{}/borrower/pre_extract/{}", folder_path, test_name.to_str().unwrap());
        let callee_fn_name = "bar";
        let caller_fn_name = "new_foo";
        let now = SystemTime::now();
        let input_borrower: Borrower = Borrower::new(
            read_to_string(file_name.as_str()).unwrap(),
            read_to_string(pre_extract_file_name.as_str()).unwrap(),
            None,
            caller_fn_name.to_string(),
            callee_fn_name.to_string(),
            read_to_string(mut_method_call_expr_file.as_str()).unwrap(),
        );
        let result  = borrow::make_borrows(
            input_borrower.into(),
        );
        let success: bool = match result {
            Ok(_) => true,
            Err(e) => {
                error!("Borrower failed: {:?}", e);
                false
            }
        };

        let time_elapsed = now.elapsed().unwrap();
        let args = vec![];
        let mut compile_cmd = compile_file(new_file_name.as_str(), &args);
        let out = compile_cmd.output().unwrap();
        println!(
            "{}: {} in {:#?}",
            (if out.status.success() {
                format!("PASSED").green()
            } else {
                format!("FAILED").red()
            }),
            test_name.to_str().unwrap(),
            time_elapsed
        );
        println!("------------------------------------------------------------------\n");

        // Update the number of failed tests.
        total_failed_tests += if !out.status.success() || !success { 1 } else { 0 };
    }

    let _ = cleanup_new_files(initial_files, current_dir);

    Ok(total_failed_tests)
}
