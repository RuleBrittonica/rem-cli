use clap::{
    Parser,
    Subcommand,
    ArgAction,
};

use std::path::PathBuf;

use crate::messages::{
    version::VERSION,
    about::ABOUT,
    author::AUTHOR,
};

#[derive(Parser)]
#[command(
    author = AUTHOR,
    version = VERSION,
    about = ABOUT
)]
pub struct REMArgs {
    #[command(subcommand)]
    pub command: REMCommands
}

#[derive(Subcommand)]
pub enum REMCommands {

    // Run the whole repairer
    Run {
        #[arg(help = "The path to the file that contains just the code that will be refactored", index = 1)]
        file_path: PathBuf,

        #[arg(help = "The path to the output file (where the refactored code ends up)", index = 2)]
        new_file_path: PathBuf,

        #[arg(help = "The name of the function that contains the code to be refactored", index = 3)]
        caller_fn_name: String,

        #[arg(help = "The name of the new function that is being extracted", index = 4)]
        callee_fn_name: String,
    },

    // Run just the controller
    Controller {
        #[arg(help = "The path to the file that contains just the code that will be refactored", index = 1)]
        file_path: PathBuf,

        #[arg(help = "The path to the output file (where the refactored code ends up)", index = 2)]
        new_file_path: PathBuf,

        #[arg(help = "The name of the function that contains the code to be refactored", index = 3)]
        caller_fn_name: String,

        #[arg(help = "The name of the new function that is being extracted", index = 4)]
        callee_fn_name: String,
    },

    //
    Borrower {

        #[arg(help = "The path to the file that contains just the code that will be refactored", index = 1)]
        file_path: PathBuf,

        #[arg(help = "The path to the output file (where the refactored code ends up)", index = 2)]
        new_file_path: PathBuf,

        #[arg(help = "The name of the function that contains the code to be refactored", index = 3)]
        caller_fn_name: String,

        #[arg(help = "The name of the new function that is being extracted", index = 4)]
        callee_fn_name: String,

        #[arg(help = "The path to the file where the mutated method will be dumped", index = 5)]
        mut_method_file_path: PathBuf,

        #[arg(help = "The path to the original file", index = 6)]
        pre_extract_file_path: PathBuf,
    },

    Repairer {

        #[arg(help = "The path to the file that contains just the code that will be refactored", index = 1)]
        file_path: PathBuf,

        #[arg(help = "The path to the output file (where the refactored code ends up)", index = 2)]
        new_file_path: PathBuf,

        #[arg(help = "The name of the function to be repaired", index = 3)]
        fn_name: String,

        /// The repairer options are as follows:
        /// * 1 => Simple
        /// * 2 => Loosest Bounds First
        /// * 3 => Tightest Bounds First
        /// * 4 => Rustfix
        ///
        /// Any other repairer selection will result in an error.
        #[arg(help = "Repairer option (1=Simple, 2=Loosest Bounds First, 3=Tightest Bounds First, 4=Rustfix)", index = 4)]
        repairer: u8,

        /// The repair method to be used:
        /// * 1 => Default
        /// * 2 => Rustc
        /// * 3 => Cargo
        // TODO Implement this lol
        // #[arg(help = "Repair Method (1=default, 2=rustc, 3=cargo)", index = 4)]
        // repair_method: u8,

        #[arg(short, long, help = "Enable verbose output", action = ArgAction::SetTrue)]
        verbose: bool,
    },

    RepairerCargo {
        src_path: PathBuf,

        manifest_path: PathBuf,

        fn_name: u8,

        /// The repairer options are as follows:
        /// * 1 => Simple
        /// * 2 => Loosest Bounds First
        /// * 3 => Tightest Bounds First
        ///
        /// Any other repairer selection will result in the program
        /// terminating.
        #[arg(help = "Repairer option (1=Simple, 2=Loosest Bounds First, 3=Tightest Bounds First)", index = 4)]
        repairer: u8,

        #[arg(short, long, help = "Enable verbose output", action = ArgAction::SetTrue)]
        verbose: bool,
    },

    /// Test all repair systems against a set of input files.
    /// Test files available from:
    /// * https://github.com/RuleBrittonica/rem-testfiles
    /// * https://github.com/RuleBrittonica/rem-cli => ./src_tests
    Test {
        #[arg(help = "Folder path containing test files")]
        folder: PathBuf,

        /// NOT YET IMPLEMENTED.
        ///
        /// CURRENTLY THE OUTPUT IS THE VERBOSE OUTPUT
        #[arg(short, long, help = "Enable verbose output", action = ArgAction::SetTrue)]
        verbose: bool
    },

    /// Same as test command, but allows you to specify a repository instead of
    /// a folder. Test files available from:
    /// * https://github.com/RuleBrittonica/rem-testfiles
    TestGithub {
        #[arg(help = "Folder path containing test files")]
        repo: String,

        /// NOT YET IMPLEMENTED.
        ///
        /// CURRENTLY THE OUTPUT IS THE VERBOSE OUTPUT
        #[arg(short, long, help = "Enable verbose output", action = ArgAction::SetTrue)]
        verbose: bool,
    },

}