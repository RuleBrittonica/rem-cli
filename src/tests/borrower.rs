use rem_borrower::borrow;
use rem_utils::compile_file;

use colored::Colorize;
use std::{
    time::SystemTime,
    fs,
    path::Path,
    io,
};

use crate::tests::utils::{
    cleanup_new_files,
    list_files_in_dir,
};

pub fn test() -> Result<(), io::Error> {
    // Capture initial state
    let current_dir: &Path = Path::new("./");
    let initial_files: Vec<String> = list_files_in_dir(current_dir)?;

    for file in fs::read_dir("./src_tests/borrower/input").unwrap() {
        let test_name = file.unwrap().file_name().to_owned();
        let file_name = format!("./src_tests/borrower/input/{}", test_name.to_str().unwrap());
        let new_file_name = format!("./src_tests/borrower/output/{}", test_name.to_str().unwrap());
        let mut_method_call_expr_file =
            format!("./src_tests/borrower/method_call_mut/{}", test_name.to_str().unwrap());
        let pre_extract_file_name = format!("./src_tests/borrower/pre_extract/{}", test_name.to_str().unwrap());
        let callee_fn_name = "bar";
        let caller_fn_name = "new_foo";
        let now = SystemTime::now();
        borrow::make_borrows(
            file_name.as_str(),
            new_file_name.as_str(),
            mut_method_call_expr_file.as_str(),
            callee_fn_name,
            caller_fn_name,
            pre_extract_file_name.as_str(),
        );
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
    }

    let _ = cleanup_new_files(initial_files, current_dir);

    Ok(())
}
