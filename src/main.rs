use clap::{Arg, Command};
use log::{
    info,
    error,
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
/// * calle_fn_name: The name of the function that contains the code to be refactored
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
            Arg::new("calle_fn_name")
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

    // Parse the input data to get it into a usable form for invocation
    let file_path = args.get_one::<String>("file_path").unwrap();
    let new_file_path = args.get_one::<String>("new_file_path").unwrap();
    let calle_fn_name = args.get_one::<String>("calle_fn_name").unwrap();
    let caller_fn_name = args.get_one::<String>("caller_fn_name").unwrap();

    // Get the refactor type, default to "default" if not provided
    let refactor_type = args.get_one::<String>("type").map(|s| s.as_str());

    // Extract the method into a new function, copy the code across, and infer
    // the function signature
    let fn_body_extraction_res: Result<(), error::ExtractFnBodyError> = extract_fn_body(file_path, new_file_path, calle_fn_name, caller_fn_name);
    match fn_body_extraction_res {
        Ok(_) => {},
        Err(e) => {
            error!("Failed to extract function body: {:?}", e);
            return;
        },
    }

    // Call the appropriate extraction method from refactor_main
    // Determine which extraction method to use based on the refactor type
    // Each of these functions handles their own logging.
    match refactor_type {
        Some("generic") => extract_function_generic(file_path, new_file_path, calle_fn_name, caller_fn_name) ,
        Some("async") => extract_function_async(file_path, new_file_path, calle_fn_name, caller_fn_name),
        None | Some("default") => extract_function(file_path, new_file_path, calle_fn_name, caller_fn_name),
        Some(other) => {
            log::error!("Unsupported refactor type: {}", other);
            std::process::exit(1);
        }
    };

    info!("Refactoring completed successfully.");

    return;

}