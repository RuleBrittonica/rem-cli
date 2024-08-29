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
mod refactor;
mod tests;


mod utils;
use utils::{
    RepairType,
    parse_repair_type,
    get_from_git,
    run_tests,
    delete_backup,
    delete_repo,
};

mod messages;

mod rem_args;
use rem_args::{
    REMArgs,
    REMCommands,
};

/// The CLI Takes the following arguments:
///
/// * file_path:  The path to the file that contains just the code that will be refactored.
///
/// The file must be structured such that it contains $0 signs where the cursors are (i.e. what text the user has selected)
/// E.g.
/// ```
/// fn foo () {
///     let n = 1;
///     $0 let m = n + 2;
///     // Calculate
///     let k = m + n;$0
///     let g = 3;
/// }
/// ```
///
/// * new_file_path: The path to the new file (i.e where we want the refactored code to end up)
/// * callee_fn_name: The name of the function that contains the code to be refactored
/// * caller_fn_name: The name of the new function
///
/// Optional arguments
/// * type:     What is being refactored. Currently supports:
///     * Extracting methods
///     * Extracting generic methods (to be implemented)
///     * Extracting methods from asynchronous code (to be implemented)
fn main() {

    logging::init_logging();

    info!("Application Started");

    let args: REMArgs = REMArgs::parse();
    let mut backup_path: Option<PathBuf> = None;

    match &args.command {
        REMCommands::Run {
            file_path,
            new_file_path,
            caller_fn_name,
            callee_fn_name
        } => {

        },

        REMCommands::Controller {
            file_path,
            new_file_path,
            caller_fn_name,
            callee_fn_name
        } => {

        },

        REMCommands::Borrower {
            file_path,
            new_file_path,
            caller_fn_name,
            callee_fn_name,
            mut_method_file_path,
            pre_extract_file_path
        } => {

        },

        REMCommands::Repairer {
            file_path,
            new_file_path,
            fn_name,
            repairer,
            verbose
        } => {
            let repair_type: RepairType = parse_repair_type(*repairer);

        },

        REMCommands::RepairerCargo {
            src_path,
            manifest_path,
            fn_name,
            repairer,
            verbose
        } => {
            let repair_type: RepairType = parse_repair_type(*repairer);
        },

        REMCommands::Test {
            folder,
            verbose // NYI
        } => {
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
    } else {
        // Handle backup path being none
        // How tf did we end up here
        error!("Backup path was never provided / saved, HOW DID WE GET HERE?");
        exit(1);
    }


    // If we have gotten this far then we know that all previous activities have
    // been successful. Delete the backup and exit successfully if that works
    info!("Refactoring completed successfully")
}
