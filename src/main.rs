use std::fs;

use clap::{Arg, Command};
use log::{
    error,
    info,
    warn,
};

mod logging;
mod error;

mod messages;
use messages::{
    about::ABOUT,
    author::AUTHOR,
    version::VERSION,
};

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
    strip_extension,
    ProgramOptions,
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

    let args = Command::new("rem-cli")
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(
            Arg::new("file_path")
                .help("The path to the file that contains just the code that will be refactored")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("new_file_path")
                .help("The path to the output file (where the refactored code ends up)")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::new("caller_fn_name")
                .help("The name of the function that contains the code to be refactored")
                .required(false)
                .index(3),
        )
        .arg(
            Arg::new("callee_fn_name")
                .help("The name of the new function that is being extracted")
                .required(false)
                .index(4),
        )
        .arg(
            Arg::new("type")
                .help("The type of refactoring - see README to learn what is currently supported. Leaving blank will run original REM extraction")
                .short('t')
                .long("type")
                .required(false),
        )
        .arg(
            Arg::new("test")
                .help("Run the tests instead of refactoring")
                .short('T')
                .long("test")
                .action(clap::ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("controller")
                .short('c')
                .long("controller")
                .action(clap::ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("borrower")
                .short('b')
                .long("borrower")
                .action(clap::ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("repairer")
                .short('r')
                .long("repairer")
                .action(clap::ArgAction::SetTrue)
                .required(false)
        )
        .get_matches();

    if args.get_flag("test") {
        info!("Running tests");

        if let Err(e) = controller::test() {
            error!("Controller tests failed: {:?}", e);
            std::process::exit(1);
        }

        if let Err(e) = borrower::test() {
            error!("Borrower tests failed: {:?}", e);
            std::process::exit(1);
        }

        if let Err(e) = repairer::test() {
            error!("Repairer tests failed: {:?}", e);
            std::process::exit(1);
        }

        info!("All tests completed");
        return;
    }

    let mut opt: ProgramOptions = ProgramOptions::All;
    {
        // Locally scoping program options
        use ProgramOptions::*;
        if args.get_flag("controller") { opt = Controller };
        if args.get_flag("borrower") { opt = Borrower };
        if args.get_flag("repairer") { opt = Repairer };

        match opt {
            Controller => info!("Running to controller"),
            Borrower => info!("Running to borrower"),
            Repairer => info!("Running to repairer"),
            All => info!("ProgramOptions Not Set - running full program"),
        };
    }

    // Parse the input data to get it into a usable form for invocation
    let file_path: &String = args.get_one::<String>("file_path").unwrap();
    let new_file_path: &String = args.get_one::<String>("new_file_path").unwrap();
    let callee_fn_name: &String = args.get_one::<String>("callee_fn_name").unwrap();
    let caller_fn_name: &String = args.get_one::<String>("caller_fn_name").unwrap();

    // Get the refactor type, default to "default" if not provided
    let refactor_type: Option<&str> = args.get_one::<String>("type").map(|s: &String| s.as_str());

    // Extract the method into a new function, copy the code across, and infer
    // the function signature
    let fn_body_extraction_res: Result<(), error::ExtractFnBodyError> = extract_fn_body(file_path, new_file_path, callee_fn_name, caller_fn_name);
    match fn_body_extraction_res {
        Ok(_) => {},
        Err(e) => {
            error!("Failed to extract function body: {:?}", e);
            return;
        },
    }

    // Backup the input file, incase the extraction fails.
    let backup: String = format!("/tmp/{}-cli-extract.bk", strip_extension(file_path));
    if let Err(e) = fs::copy(file_path, &backup) {
        error!("Failed to create backup in main: {:?}", e);
        warn!("Returning early, extraction will not proceed");
        return;
    }

    // Call the appropriate extraction method from refactor_main
    // Determine which extraction method to use based on the refactor type
    // Each of these functions handles their own logging.
    let success: bool = match refactor_type {
        Some("generic") => extract_function_generic(file_path, new_file_path, callee_fn_name, caller_fn_name, &backup, opt),
        Some("async") => extract_function_async(file_path, new_file_path, callee_fn_name, caller_fn_name, &backup, opt),
        None | Some("default") => extract_function(file_path, new_file_path, callee_fn_name, caller_fn_name, &backup, opt),
        Some(other) => {
            log::error!("Unsupported refactor type: {}", other);
            std::process::exit(1);
        }
    };

    if success {
        info!("Refactoring completed successfully.");
    } else {
        warn!("Refactoring failed for some reason. Check logs to work it out!");

        // If we fail the extraction for any reason, the original file and new
        // files are overwritten with the backup.
        if let Err(e) = fs::copy(&backup, file_path) {
            error!("Failed to restore backup file in main: {:?}", e);
        }
        if let Err(e) = fs::copy(&backup, new_file_path) {
            error!("Failed to overwrite new file with backup after failed extraction: {:?}", e);
        }
        warn!("Input file path has been overwritten with the input file");
        warn!("Output file path has been overwritten with the input file");
    }

    return;

}