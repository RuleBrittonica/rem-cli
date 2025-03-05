//! The goal of this module is to allow the user to convert the project from a
//! Rust project to a .llbc file. This is done by using the CHARON tool.
//! The CHARON tool is located in the `tools/charon` directory, as outlined by
//! the Config.toml file.
//! The program now relies on cli inputs, then environment variables, then the
//! paths specified in the Config.toml file to locate the CHARON and AENEAS
//! tools. This is outlined in rem-utils.
//!
//! This process is run both before the refactor process, to generate a .llbc of
//! the original project, and after the refactor process, to generate a .llbc of
//! the refactored project.
//! At some point it may need to be modified to allow for incremental
//! compilation. But at this stage, the goal is to get the basic functionality
//! working first.
//!
//! There is the potential to use --input to specify just an input file.
//! This module is structured a little weirdly. I wanted to have a file that
//! points to both the conversion and verification functions. However, because
//! the conversion is very straightforward (and done locally), and the
//! verification is done externally, this gets a lot more complicated :(
//! Eventually, I would like to call out to both CHARON and AENEAS as modules
//! instead of through the CLI. However, they are currently written in ocaml so
//! this does create some issues 

use std::{
    path::PathBuf,
    process::Command,
};
use config::{
    Config,
    File,
};
use crate::local_config::Settings;
use log::{
    info,
    error,
};

use rem_utils::resolve_charon_path;
use rem_verification::convert::coq_conversion;

/// This is the public method that is called to convert the project to a .llbc
/// file. It takes in the path to the project. From there it works out the path
/// to the CHARON tool, and then runs the CHARON tool to convert the project to
/// a .llbc file.
/// The CHARON tool is run as follows: `charon --hide-marker-traits <path_to_project>`
/// We can also specify the output folder using the --dest flag. This is an
/// optional input to the method.
pub fn local_llbc_conversion(
    project_path: &PathBuf, // Alternatively this is the file path.
    output_path: &PathBuf,
    charon_path: &Option<PathBuf>,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Get the path to the CHARON tool.
    let charon_path: PathBuf = resolve_charon_path(charon_path)?;

    // Build the command
    let mut cmd = Command::new(&charon_path);
    cmd.arg("--hide-marker-traits")
        .arg("--input")
        .arg(project_path)
        .arg("--dest-file")
        .arg(output_path);

    // Run the command and capture its output.
    let output = cmd.output()?;
    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("{}", format!(
            "CHARON tool failed:\nstdout: {}\nstderr: {}",
            stdout, stderr
        ));
        return Err(format!(
            "CHARON tool failed:\nstdout: {}\nstderr: {}",
            stdout, stderr
        ).into());
    }

    // Return the path to the output file.
    Ok(output_path.to_path_buf())
}

/// Returns the paths to the original and new .v (CoQ) files.
/// Local hook into rem-verifications convert::coq_conversion function.
pub fn local_coq_conversion(
    original_llbc: &PathBuf,
    refactored_llbc: &PathBuf,
    out_dir: &Option<PathBuf>,
    aeneas_path: &Option<PathBuf>,
) -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    coq_conversion(original_llbc, refactored_llbc, out_dir, aeneas_path)
}