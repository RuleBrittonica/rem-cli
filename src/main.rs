use std::{
    path::PathBuf,
    process::exit,
};

use clap::Parser;

use log::{
    error,
    info,
};

mod logging;
mod error;
mod tests;

mod refactor;
use refactor::{
    extract_function::local_extract_method,
    non_local_controller::non_local_controller,
    borrow::borrow,
};

use rem_repairer::{
    common::{
        RepairResult,
        RepairSystem,
    },
    repair_lifetime_loosest_bound_first,
    repair_lifetime_simple,
    repair_lifetime_tightest_bound_first,
    repair_rustfix
};

mod utils;
use utils::{
    delete_backup,
    delete_repo,
    get_from_git,
    handle_result,
    run_tests,
    backup_file,
};

mod messages;

mod rem_args;
use rem_args::{
    REMArgs,
    REMCommands,
};

use crate::refactor::throughput::{
    Extract,
    Controller,
    Borrower,
    Repairer,
};

mod local_config;
use local_config::Settings;

mod prefactor;
use prefactor::convert_to_llbc::{self, convert_to_llbc};

#[derive(Debug, PartialEq, Eq)]
enum ProgramOptions{
    Refactoring,
    Testing,
    CargoRepairing,
}

/// The CLI Takes the following arguments:
fn main() -> Result<(), Box<dyn std::error::Error>> {

    logging::init_logging();

    info!("Application Started");

    let args: REMArgs = REMArgs::parse();
    let mut backup_path: Option<PathBuf> = None;

    // Set to refactoring by default. This will be changed by the rest of the
    // CLI if a non-refactoring program is run.
    let mut prog_run: ProgramOptions = ProgramOptions::Refactoring;

    match &args.command {
        REMCommands::Run {
            file_path,
            new_fn_name,
            start_index,
            end_index,
            verbose,
        } => {
            // Create our backup
            backup_path = backup_file(file_path.clone());

            let file_path = file_path.to_str().expect("Path is not valid UTF-8");

            todo!("Run is not yet implemented");
        },

        REMCommands::RunShort {
            file_path,
            new_fn_name,
            start_index,
            end_index,
            verbose
        } => {
            if *verbose {
                info!("Running RunShort in verbose mode");
                info!("File Path: {:?}", file_path);
                info!("New Function Name: {:?}", new_fn_name);
                info!("Start Index: {:?}", start_index);
                info!("End Index: {:?}", end_index);
            }

            // We have a few things to do here
            // TODO
            // 1. Create the backup
            backup_path = backup_file(file_path.clone());
            // 2. Create the original llbc file
            // We need to determine where to save the output file. This will
            // depend on if the user has specified a project path or a file
            // path. If the user has specified a dir, assume that the output
            // should go in there under dir/dir_name.llbc
            // If the user has specified a file, assume that the output should
            // go in the same directory as the file, with the same name as the
            // file, but with a .llbc extension.

            let out_path: PathBuf = match file_path.is_dir() {
                true => {
                    let mut out_path = file_path.clone();
                    out_path.push(file_path.file_name().unwrap());
                    out_path.set_extension("llbc");
                    out_path
                },
                false => {
                    let mut out_path = file_path.clone();
                    out_path.set_extension("llbc");
                    out_path
                }
            };

            match convert_to_llbc(file_path, &out_path) {
                Ok(output_path) => {
                    // Verify that there is a file at the output path.
                    if output_path.exists() {
                        info!("Conversion to LLBC succeeded for project: {:?}", file_path);
                    } else {
                        error!("Conversion to LLBC failed: No file found at output path: {:?}", output_path);
                        return Err("Conversion to LLBC failed: No file found at output path".into());
                    }
                },
                Err(e) => {
                    error!("Conversion to LLBC failed: {}", e);
                    return Err(e);
                }
            }

            // 3. Extract the method
            // 4. Write the new code to the original file
            // 5. Create the new llbc file
            // 6. Convert the original and new llbc files to CoQ
            // 7. Verify the original and new CoQ files
            // 8. Ensure success and cleanup.
            // 9. Print out the results
            todo!("RunShort is not yet implemented");
        },

        REMCommands::Extract {
            file_path,
            new_fn_name,
            start_index,
            end_index,
            verbose
        } => {
            if *verbose {
                info!("File Path: {:?}", file_path);
                info!("New Function Name: {}", new_fn_name);
                info!("Start Index: {}", start_index);
                info!("End Index: {}", end_index);
                info!("Verbose Mode: {}", if *verbose { "yes" } else { "no" });
            } else {
                info!("Verbose Mode: {}", if *verbose { "yes" } else { "no" });
            }

            let start_idx: u32 = *start_index as u32;
            let end_idx: u32 = *end_index as u32;

            let input_extract: Extract = Extract::new(
                file_path.to_path_buf(),
                None, // output code not yet populated
                new_fn_name.to_string(),
                start_idx,
                end_idx,
                None, // caller_fn_name not yet populated
            );

            let result = local_extract_method(input_extract);
            // Unwrap the result, if success, log the new code and the
            // caller_fn_name
            let success: bool = match result {
                Ok(result) => {
                    let output_code: String = result.get_output_code().unwrap();
                    let caller_fn_name: String = result.get_caller_fn_name().unwrap();
                    info!("New Code: \n{}", output_code);
                    info!("Caller Function Name: {}", caller_fn_name);
                    true
                }
                Err(e) => {
                    error!("Extraction failed: {:?}", e);
                    false
                }
            };
        }

        REMCommands::Controller {
            file_path,
            new_file_path,
            caller_fn_name,
            callee_fn_name
        } => {
            // Create our backup
            backup_path = backup_file(file_path.clone());

            let file_path: &str = file_path.to_str().expect("Path is not valid UTF-8");
            let new_file_path: &str = new_file_path.to_str().expect("Path is not valid UTF-8");

            let controller: Controller = todo!();

            let result: Result<String, error::RefactoringError>  = non_local_controller(
                controller,
            );

            // TODO Handle the result
            let success: bool = match result {
                Ok(_) => true,
                Err(e) => {
                    error!("Controller failed: {:?}", e);
                    false
                }
            };

            handle_result(
                success,
                "Controller",
                &format!(
                    "Controller was run on its own with file_path: {} | new_file_path: {} | caller_fn_name: {} | callee_fn_name: {}",
                    file_path,
                    new_file_path,
                    caller_fn_name,
                    callee_fn_name,
                    ),
            )

        },

        REMCommands::Borrower {
            file_path,
            new_file_path,
            caller_fn_name,
            callee_fn_name,
            mut_method_file_path,
            pre_extract_file_path
        } => {
            // Create our backup
            backup_path = backup_file(file_path.clone());

            let file_path: &str = file_path.to_str().expect("Path is not valid UTF-8");
            let new_file_path: &str = new_file_path.to_str().expect("Path is not valid UTF-8");
            let mut_method_file_path: &str = mut_method_file_path.to_str().expect("Path is not valid UTF-8");
            let pre_extract_file_path: &str = pre_extract_file_path.to_str().expect("Path is not valid UTF-8");

            let input_borrower: Borrower = todo!();
            let result  = borrow(
                input_borrower,
                // file_path,
                // new_file_path,
                // callee_fn_name,
                // caller_fn_name,
                // mut_method_file_path,
                // pre_extract_file_path,
            );
            let success: bool = match result {
                Ok(_) => true,
                Err(e) => {
                    error!("Borrower failed: {:?}", e);
                    false
                }
            };

            handle_result(success,
                "Borrower",
                &format!(
                    "Borrower was run on its own with file_path: {} | new_file_path: {} | caller_fn_name: {} | callee_fn_name: {} | mut_method_file_path: {} | pre_extract_file_path: {}",
                    file_path,
                    new_file_path,
                    caller_fn_name,
                    callee_fn_name,
                    mut_method_file_path,
                    pre_extract_file_path,
                ),
            )

        },

        REMCommands::Repairer {
            file_path,
            new_file_path,
            fn_name,
            repairer,
            verbose, // TODO Implement this
        } => {
            // Create our backup
            backup_path = backup_file(file_path.clone());

            let file_path: &str = file_path.to_str().expect("Path is not valid UTF-8");
            let new_file_path: &str = new_file_path.to_str().expect("Path is not valid UTF-8");
            let repair_system: &dyn RepairSystem = match repairer {
                1 => &repair_lifetime_simple::Repairer {},
                2 => &repair_lifetime_loosest_bound_first::Repairer {},
                3 => &repair_lifetime_tightest_bound_first::Repairer {},
                4 => &repair_rustfix::Repairer {},
                _ => {
                    error!("{} is not a valid option for the repair system", *repairer);
                    exit(1)
                },
            };

            let RepairResult { success, .. } = repair_system.repair_function(
                file_path,
                new_file_path,
                fn_name
            );

            handle_result(success,
                "Repairer",
                &format!(
                    "Repairer was run on its own with file_path: {} | new_file_path: {} | fn_name: {} | repair_system: {}",
                    file_path,
                    new_file_path,
                    fn_name,
                    repair_system.name(),
                ),
            )
        },

        REMCommands::RepairerCargo {
            src_path,
            manifest_path,
            fn_name,
            repairer,
            verbose
        } => {
            prog_run = ProgramOptions::CargoRepairing;
            todo!("RepairerCargo is not yet implemented");

        },

        REMCommands::ConvertToCoq {
            original_llbc,
            refactored_llbc,
            out_dir,
            verbose
        } => {
            todo!("ConvertToCoQ is not yet implemented");
        },

        REMCommands::ConvertToLLBC {
            project_path,
            out_path,
            verbose,
        } => {
            if *verbose {
                info!("Starting conversion to LLBC for project: {:?}", project_path);
            }
            // Call the conversion function.
            match convert_to_llbc(project_path, out_path) {
                Ok(output_path) => {
                    // Verify that there is a file at the output path.
                    if output_path.exists() {
                        info!("Conversion to LLBC succeeded for project: {:?}", project_path);
                    } else {
                        error!("Conversion to LLBC failed: No file found at output path: {:?}", output_path);
                        return Err("Conversion to LLBC failed: No file found at output path".into());
                    }
                },
                Err(e) => {
                    error!("Conversion to LLBC failed: {}", e);
                    return Err(e);
                }
            }

        },

        REMCommands::Verify {
            original_coq,
            refactored_coq,
            top_level_function,
            verbose
        } => {
            todo!("Verify is not yet implemented");
        },

        REMCommands::Test {
            folder,
            verbose // NYI
        } => {
            prog_run = ProgramOptions::Testing;

            if *verbose {
                info!("Running tests in verbose mode");
            } else {
                info!("Running tests");
            }

            match run_tests(folder.clone()) {
                Ok(x) => info!("Test running finished, {} tests failed", x),
                Err(e) => {
                    error!("Test running failed: {:?}", e);
                    exit(1);
                }
            }
        },

        REMCommands::TestGithub {
            repo,
            verbose, // NYI
        } => {
            prog_run = ProgramOptions::Testing;

            if *verbose {
                info!("Running tests in verbose mode from GitHub repo: {}", repo.clone());
            } else {
                info!("Running tests from GitHub repo: {}", repo.clone());
            }

            let path: PathBuf = match get_from_git(repo.clone()) {
                Ok(p) => p,
                Err(e) => {
                    error!("Failed to fetch from GitHub: {}", e);
                    exit(1);
                }
            };

            match run_tests(path.clone()) {
                Ok(failed_tests) => info!("Test running finished, {} tests failed", failed_tests),
                Err(e) => {
                    error!("Test running failed: {:?}", e);
                    exit(1);
                }
            }

            match delete_repo(path) {
                Ok(_) => info!("Successfully deleted folder downloaded from Git"),
                Err(e) => {
                    error!("Unable to delete downloaded folder: {}", e);
                    exit(1);
                }
            }
        },
    }

    // Attempt to delete the backup
    if let Some(backup_path_real) = backup_path {
        if let Err(e) = delete_backup(backup_path_real) {
            error!("Failed to delete backup: {:?}", e);
            exit(1);
        } else {
            info!("Backup deleted successfully");
        }
    } else if prog_run == ProgramOptions::Refactoring {
        // Handle backup path being none -
        // How tf did we end up here
        error!("Backup path was never provided / saved, HOW DID WE GET HERE?");
        exit(1);
    }

    // If we have gotten this far then we know that all previous activities have
    // been successful. Delete the backup and exit successfully if that works
    info!("Refactoring completed successfully");

    Ok(())
}
