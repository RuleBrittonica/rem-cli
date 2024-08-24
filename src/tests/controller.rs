use rem_utils::compile_file;
use rem_controller::non_local_controller;
use std::{
    fs,
    time::SystemTime,
};
use colored::Colorize;
use std::io;

pub fn test() -> Result<(), io::Error> {
    for file in fs::read_dir("./src_tests/controller/input")? {
        let file = file?;
        let test_name = file.file_name().to_owned();
        let file_name = format!("./src_tests/controller/input/{}", test_name.to_str().unwrap());
        let new_file_name = format!("./src_tests/controller/output/{}", test_name.to_str().unwrap());
        let callee_fn_name = "bar";
        let caller_fn_name = "new_foo";
        let now = SystemTime::now();
        let success = non_local_controller::make_controls(
            file_name.as_str(),
            new_file_name.as_str(),
            callee_fn_name,
            caller_fn_name,
        );
        let time_elapsed = now.elapsed().unwrap();
        let args = vec![];
        let mut compile_cmd = compile_file(new_file_name.as_str(), &args);
        let out = compile_cmd.output()?;
        println!(
            "{}: {} in {:#?}",
            (if out.status.success() && success {
                format!("PASSED").green()
            } else {
                format!("FAILED").red()
            }),
            test_name.to_str().unwrap(),
            time_elapsed
        );
        println!("------------------------------------------------------------------\n");
    }
    Ok(())
}
