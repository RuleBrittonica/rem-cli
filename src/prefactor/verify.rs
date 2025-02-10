//! Local hooks to the conversion and verification processes of
//! rem-verification.

use std::path::PathBuf;

use rem_verification::verify::coq_verification;

/// Returns the paths to:
/// - The _CoQProject file
/// - The EquivCheck.v file
/// - The Primitives.v file
pub fn local_coq_verification(
    original_coq: &PathBuf,
    refactored_coq: &PathBuf,
    top_level_function: &String,
) -> Result<( PathBuf, PathBuf, PathBuf, bool ), Box<dyn std::error::Error>> {
    // TODO - We want to call the local function, not reach out to the CLI of rem-verification
    coq_verification( original_coq, refactored_coq, top_level_function )

    // Call rem-verification as a binary. This is a temporary solution.
    // Capture the outputs (being the paths to the _CoQProject, EquivCheck.v,
    // and Primitives.v files)
    // The exit code is also captured, which is used to determine if the
    // verification was a success or not.
    // let config: Config = Config::builder()
    //     .add_source(File::with_name("Config")
    //     .required(true))
    //     .build()?;

    // let s: Settings = config.try_deserialize()?;

    // let rem_verification_path = PathBuf::from(s.programs.rem_verification);

    // let output = Command::new(rem_verification_path)
    //     .arg("run")
    //     .arg(original_coq)
    //     .arg(refactored_coq)
    //     .arg(top_level_function)
    //     .output()?;

    // let success = output.status.success();

    // // We know rem-verification prints markers to stderr:
    // //   COQ_PROJECT: <path>
    // //   EQUIVCHECK: <path>
    // //   PRIMITIVES: <path>
    // let stderr_str = String::from_utf8_lossy(&output.stderr);

    // // Variables to hold the extracted file paths.
    // let mut coq_project_path: Option<String> = None;
    // let mut equivcheck_path: Option<String> = None;
    // let mut primitives_path: Option<String> = None;

    // // Parse each line from stderr.
    // for line in stderr_str.lines() {
    //     if line.starts_with("COQ_PROJECT:") {
    //         coq_project_path = Some(line.trim_start_matches("COQ_PROJECT:").trim().to_string());
    //     } else if line.starts_with("EQUIVCHECK:") {
    //         equivcheck_path = Some(line.trim_start_matches("EQUIVCHECK:").trim().to_string());
    //     } else if line.starts_with("PRIMITIVES:") {
    //         primitives_path = Some(line.trim_start_matches("PRIMITIVES:").trim().to_string());
    //     }
    // }

    // // Check that we found all required markers.
    // if coq_project_path.is_none() || equivcheck_path.is_none() || primitives_path.is_none() {
    //     return Err("Failed to parse output from rem-verification: Not all file paths were found.".into());
    // }

    // // Convert the parsed strings into PathBufs.
    // let coq_project = PathBuf::from(coq_project_path.unwrap());
    // let equivcheck = PathBuf::from(equivcheck_path.unwrap());
    // let primitives = PathBuf::from(primitives_path.unwrap());

    // Ok((coq_project, equivcheck, primitives, success))

}