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

pub mod refactor;
use refactor::{
    extract_function::local_extract_method,
    extract_function::replace_text_in_file,
    non_local_controller::non_local_controller,
    borrow::borrow,
    throughput::{
        Extract,
        Controller,
        Borrower,
        Repairer,
    },
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
    delete_repo,
    delete_files,
    get_from_git,
    handle_result,
    run_tests,
    backup_file,
};

mod commands;
mod messages;

mod rem_args;
use rem_args::{
    REMArgs,
    REMCommands,
};

mod local_config;
use local_config::Settings;

pub mod prefactor;
use prefactor::{
    convert::local_llbc_conversion,
    convert::local_coq_conversion,
    verify::local_coq_verification,
};

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

            let file_path: &str = file_path.to_str().expect("Path is not valid UTF-8");

            todo!("Run is not yet implemented");
        },

        REMCommands::RunShort {
            file_path,
            new_fn_name,
            start_index,
            end_index,
            verbose,
            cleanup,
        } => {
            // backup_path = backup_file(file_path.clone());
            commands::run_short::run_short(file_path, new_fn_name, start_index, end_index, verbose, cleanup)?;

        },

        REMCommands::Extract {
            file_path,
            new_fn_name,
            start_index,
            end_index,
            verbose
        } => {
            commands::extract::
                extract(file_path, new_fn_name, start_index, end_index, verbose)?;
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
            commands::convert::convert_to_coq(original_llbc, refactored_llbc, out_dir, verbose)?;
        },

        REMCommands::ConvertToLLBC {
            project_path,
            out_path,
            verbose,
        } => {
            commands::convert::convert_to_llbc(project_path, out_path, verbose)?;
        },

        REMCommands::Verify {
            original_coq,
            refactored_coq,
            top_level_function,
            verbose
        } => {
            commands::verify::verify(original_coq, refactored_coq, top_level_function, verbose)?;
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

    // If we have gotten this far then we know that all previous activities have
    // been successful. Delete the backup and exit successfully if that works
    info!("Refactoring completed successfully");

    Ok(())
}
