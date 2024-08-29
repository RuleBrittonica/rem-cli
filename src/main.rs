use std::fs;

use clap::{
    Arg,
    Command,
    Parser,
};

use log::{
    debug,
    error,
    info,
    warn
};

mod logging;
mod error;

mod prefactor;
use prefactor::extract_body::extract_fn_body;

mod refactor;
use refactor::refactor_main::{
    extract_function,
    extract_function_generic,
    extract_function_async,
};

mod tests;
use tests::{
    controller,
    borrower,
    repairer,
};

mod utils;
use utils::{
    // strip_extension,
    ProgramOptions,
};

mod messages;

mod rem_args;
use rem_args::{
    REMArgs,
    REMCommands,
};

use rem_repairer::{
    common::{
        RepairSystem,
        DebugRepairSystem,
    },
    repair_lifetime_loosest_bound_first,
    repair_lifetime_simple,
    repair_lifetime_tightest_bound_first
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

    match &args.command {
        REMCommands::Run {
            file_path,
            new_file_path,
            caller_fn_name,
            callee_fn_name
        } => {

        }

        REMCommands::Controller {
            file_path,
            new_file_path,
            caller_fn_name,
            callee_fn_name
        } => {

        }

        REMCommands::Borrower {
            file_path,
            new_file_path,
            caller_fn_name,
            callee_fn_name,
            mut_method_file_path,
            pre_extract_file_path
        } => {

        }

        REMCommands::Repairer {
            file_path,
            new_file_path,
            fn_name,
            repairer,
            verbose
        } => {

        }

        REMCommands::RepairerCargo {
            src_path,
            manifest_path,
            fn_name,
            repairer,
            verbose
        } => {

        }

        REMCommands::Test {
            folder,
            verbose
        } => {

        }

        REMCommands::TestGithub {
            repo,
            verbose
        } => {

        }
    }
}
