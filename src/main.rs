use std::{
    process::exit,
    path::PathBuf,
};

use clap::Parser;

use log::{
    error,
    info,
};

mod logging;
mod error;
mod prefactor;
mod tests;

mod refactor;
use refactor::{
    non_local_controller::non_local_controller,
    borrow::borrow,
    repair_lifetime::{
        repair_lifetime,
        repair_lifetime_rustc,
        repair_lifetime_cargo,
    },
};

use rem_repairer::{
    common::{
        RepairResult,
        RepairSystem,
    },
    repair_lifetime_simple,
    repair_lifetime_loosest_bound_first,
    repair_lifetime_tightest_bound_first,
};

mod utils;
use utils::{
    delete_backup,
    delete_repo,
    get_from_git,
    handle_result,
    parse_repair_type,
    run_tests,
    RepairType,
    backup_file,
};

mod messages;

mod rem_args;
use rem_args::{
    REMArgs,
    REMCommands,
};

#[derive(Debug, PartialEq, Eq)]
enum ProgramOptions{
    Refactoring,
    Testing,
    CargoRepairing,
}

/// The CLI Takes the following arguments:
fn main() {

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
            new_file_path,
            caller_fn_name,
            callee_fn_name
        } => {
            // Create our backup
            backup_path = backup_file(file_path.clone());

            let file_path = file_path.to_str().expect("Path is not valid UTF-8");
            let new_file_path = new_file_path.to_str().expect("Path is not valid UTF-8");
        },

        REMCommands::Controller {
            file_path,
            new_file_path,
            caller_fn_name,
            callee_fn_name
        } => {
            // Create our backup
            backup_path = backup_file(file_path.clone());

            let file_path = file_path.to_str().expect("Path is not valid UTF-8");
            let new_file_path = new_file_path.to_str().expect("Path is not valid UTF-8");

            let success: bool = non_local_controller(
                file_path,
                new_file_path,
                callee_fn_name,
                caller_fn_name,
            );

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

            let success: bool = borrow(
                file_path,
                new_file_path,
                callee_fn_name,
                caller_fn_name,
                mut_method_file_path,
                pre_extract_file_path,
            );

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
            let repair_type: RepairType = parse_repair_type(*repairer);
            let repair_system: &dyn RepairSystem = match repair_type {
                RepairType::Simple => &repair_lifetime_simple::Repairer {},
                RepairType::LoosestBoundsFirst => &repair_lifetime_loosest_bound_first::Repairer {},
                RepairType::TightestBoundsFirst => &repair_lifetime_tightest_bound_first::Repairer {},
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

            let repair_type: RepairType = parse_repair_type(*repairer);
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
    } else if prog_run != ProgramOptions::Refactoring {
        // Handle backup path being none -
        // How tf did we end up here
        error!("Backup path was never provided / saved, HOW DID WE GET HERE?");
        exit(1);
    }


    // If we have gotten this far then we know that all previous activities have
    // been successful. Delete the backup and exit successfully if that works
    info!("Refactoring completed successfully")
}
