//! The goal of this module is to allow the user to convert the project from a
//! Rust project to a .llbc file. This is done by using the CHARON tool.
//! The CHARON tool is located in the `tools/charon` directory, as outlined by
//! the Config.toml file.
//!
//! This process is run both before the refactor process, to generate a .llbc of
//! the original project, and after the refactor process, to generate a .llbc of
//! the refactored project.
//! At some point it may need to be modified to allow for incremental
//! compilation. But at this stage, the goal is to get the basic functionality
//! working first.
//!
//! There is the potential to use --input to specify just an input file.

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
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Get the path to the CHARON tool.
    let charon_path = get_charon_path()?;

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

/// Gets the path to the CHARON tool. This is done by reading the Config.toml
/// file and getting the path to the CHARON tool from there.
fn get_charon_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config = Config::builder()
        // "Config" here means it will look for a file named "Config.toml" by default.
        .add_source(File::with_name("Config")
        .required(true))
        .build()?;

    let s: Settings = config.try_deserialize()?;
    // Convert the charon field (a String) into a PathBuf in one step.
    let charon_path = PathBuf::from(s.programs.charon);
    Ok(charon_path)
}

/// Returns the paths to the original and new .v (CoQ) files.
/// Local hook into rem-verifications convert::coq_conversion function.
pub fn local_coq_conversion(
    original_llbc: &PathBuf,
    refactored_llbc: &PathBuf,
    out_dir: &Option<PathBuf>,
) -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    coq_conversion(original_llbc, refactored_llbc, out_dir)
}