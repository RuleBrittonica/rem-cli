use clap::{Arg, Command};

/*
One potential design for the CLI is to take the following arguments:

file_path:  The path to the file that contains just the code that will be refactored.
target_var: The name of the variable/function/struct/impl to be refactored
new_name:   The new name for the variable/function/... that is being refactored

Optional arguments
- out_path: If you want a new file to be created instead of just overwriting the original file
- type:     What is being refactored (i.e. variable/function/struct/impl/trait)


*/

mod messages;
use crate::messages::{
    about::ABOUT,
    author::AUTHOR,
    help::HELP,
    version::VERSION,
};

fn main() {
    let matches = Command::new("rem-cli")
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(
            Arg::new("file_path")
                .help("The path to the file that contains just the code that will be refactored")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("target_var")
                .help("The name of the variable/function/struct/impl to be refactored")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("new_name")
                .help("The new name for the variable/function/... that is being refactored")
                .required(true)
                .index(3),
        )
        .arg(
            Arg::new("out_path")
                .help("Optional: Path to create a new file instead of overwriting the original file")
                .short('o')
                .long("out"),
        )
        .arg(
            Arg::new("type")
                .help("Optional: The type of refactoring (e.g., variable, function, struct, impl, trait)")
                .short('t')
                .long("type"),
        )
        .get_matches();

        // Retrieve values from matches
        let file_path = matches.get_one::<String>("file_path").expect("File path is required");
        let target_var = matches.get_one::<String>("target_var").expect("Target variable is required");
        let new_name = matches.get_one::<String>("new_name").expect("New name is required");
        let out_path = matches.get_one::<String>("out_path");
        let refactor_type = matches.get_one::<String>("type");

        // Example of how to use the arguments
        println!("File path: {}", file_path);
        println!("Target variable: {}", target_var);
        println!("New name: {}", new_name);

        if let Some(out_path) = out_path {
            println!("Output path: {}", out_path);
            // Handle file creation or modification based on out_path
        } else {
            println!("No output path specified, will overwrite the original file.");
            // Handle in-place file modification
        }

        if let Some(refactor_type) = refactor_type {
            println!("Refactor type: {}", refactor_type);
            // Handle different types of refactoring
        }

        // Read and process the file
        let content = std::fs::read_to_string(std::path::Path::new(file_path))
            .expect(&format!("Failed to read file at path: {}", file_path));
        println!("File content:\n{}", content);
}