use rem_repairer::{
    common::{
        RepairResult,
        RepairSystem,
    },
    repair_lifetime_loosest_bound_first,
    repair_lifetime_simple,
    repair_lifetime_tightest_bound_first,
    repair_rustfix,
};
use colored::Colorize;
use std::{
    io,
    iter::zip,
    path::{
        Path,
        PathBuf
    },
    time::SystemTime

};

use log::{
    info,
    error,
};

use crate::tests::utils::{
    cleanup_new_files,
    list_files_in_dir,
};

pub fn test(path: PathBuf) -> Result<u8, io::Error> {

    let folder_path: String = match path.to_str() {
        Some(path_str) => path_str.to_string(),
        None => {
            error!("Failed to convert path to string: {:?}", path);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid path"));
        }
    };

    // The path provided must be a directory
    if !path.is_dir() {
        error!("The path provided is not a directory: {}", folder_path);
        return Err(io::Error::new(io::ErrorKind::NotFound, "Path is not a directory"));
    }

    info!("Running tests from directory {}{}", folder_path, "repairer");

    let file_names: Vec<&str> = vec![
        "borrow",
        "in_out_lifetimes",
        "lifetime_bounds",
        "in_out_lifetimes_original_extract",
        "lifetime_bounds_not_enough_annotations",
        "in_out_lifetimes_wide_bounds",
        "bounds_elider",
    ];

    let function_sigs: Vec<(&str, &str)> = vec![
        ("", ""),
        (
            "bar_extracted",
            "fn bar_extracted(x_ref: &i32, z: &i32, y: &i32) -> &i32",
        ),
        ("bar_extracted", "fn bar_extracted(p: &mut & i32, x: & i32)"),
        ("", ""),
        ("", ""),
        (
            "bar_extracted",
            "fn bar_extracted<'a, 'b, 'c>(x_ref: &'a i32, z: &'b i32, y: &'c i32) -> &'a i32 {",
        ),
        ("bar", ""),
    ];

    let repair_systems: Vec<&dyn RepairSystem> = vec![
        &repair_lifetime_simple::Repairer {},
        &repair_rustfix::Repairer {},
        &repair_lifetime_tightest_bound_first::Repairer {},
        &repair_lifetime_loosest_bound_first::Repairer {},
    ];

    // For deleting the large amount of files that REM creates.
    let current_dir: &Path = Path::new("./");
    let initial_files: Vec<String> = list_files_in_dir(current_dir)?;

    let mut total_failed_tests: u8 = 0;

    for (file_name, (fn_name, _)) in zip(file_names, function_sigs) {
        for repair_system in repair_systems.iter() {

            // Format input file name correctly
            let input_file_name = format!("{}/repairer/input/{}.rs", folder_path, file_name);

            // Format output file name correctly
            let output_file_name = format!("{}/repairer/output/{}_{}.rs", folder_path, file_name, repair_system.name());

            // Perform the repair operation
            let success = print_repair_stat(
                repair_system,
                input_file_name.as_str(),
                output_file_name.as_str(),
                fn_name,
            )?;

            if !success {
                total_failed_tests += 1;
            }
        }
        println!("------------------------------------------------------------------");
    }

    let _ = cleanup_new_files(initial_files, current_dir)?;

    Ok(total_failed_tests)
}

fn print_repair_stat(
    repair_system: &&dyn RepairSystem,
    file_name: &str,
    new_file_name: &str,
    fn_name: &str,
) -> Result<bool, io::Error> {
    println!("\n\n{}: {}", file_name, fn_name);
    let now = SystemTime::now();
    let RepairResult { success, .. } =
        repair_system.repair_function(file_name, new_file_name, fn_name);
    let time_elapsed = now.elapsed().unwrap();
    println!(
        "{}: {} refactored {} in {:#?}",
        (if success {
            format!("PASSED").green()
        } else {
            format!("FAILED").red()
        }),
        repair_system.name(),
        file_name,
        time_elapsed
    );
    Ok(success)
}


// fn print_repair_stat_project(
//     repair_system: &&dyn RepairSystem,
//     src_path: &str,
//     manifest_path: &str,
//     fn_name: &str,
// ) -> bool {
//     println!("\n\n{}: {}", src_path, fn_name);
//     let now = SystemTime::now();
//     let RepairResult { success, .. } =
//         repair_system.repair_project(src_path, manifest_path, fn_name);
//     let time_elapsed = now.elapsed().unwrap();
//     println!(
//         "{}: {} refactored {} in {:#?}",
//         (if success {
//             format!("PASSED").green()
//         } else {
//             format!("FAILED").red()
//         }),
//         repair_system.name(),
//         src_path,
//         time_elapsed
//     );
//     success
// }
