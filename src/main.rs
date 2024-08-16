use clap::{Arg, Command};

/*
One potential design for the CLI is to take the following arguments:

file_path:  The path to the file that contains just the code that will be refactored.
target_var: The name of the variable/function/struct/impl to be refactored
new_name:   The new name for the variable/function/... that is being refactored

Optional arguments
- out_path: If you want a new file to be created instead of just overwriting the original file


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
        )
        .arg(
            Arg::new("target_var")
        )
        .arg(
            Arg::new("new_name")
        )
        .arg(
            Arg::new("out_path")
                .short('o')
                .long("out"),
        );
}