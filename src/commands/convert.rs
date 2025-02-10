use std::path::PathBuf;

use crate::local_llbc_conversion;
use crate::local_coq_conversion;
use log::{
    info, error
};

pub fn convert_to_llbc(
    project_path: &PathBuf,
    out_path: &PathBuf,
    verbose: &bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if *verbose {
        info!("Starting conversion to LLBC for project: {:?}", project_path);
    }
    // Call the conversion function.
    match local_llbc_conversion(project_path, out_path) {
        Ok(output_path) => {
            // Verify that there is a file at the output path.
            if output_path.exists() {
                info!("Conversion to LLBC succeeded for project: {:?}", project_path);
            } else {
                error!("Conversion to LLBC failed: No file found at output path: {:?}", output_path);
                return Err("Conversion to LLBC failed: No file found at output path".into());
            }
        },
        Err(e) => {
            error!("Conversion to LLBC failed: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

pub fn convert_to_coq(
    original_llbc: &PathBuf,
    refactored_llbc: &PathBuf,
    out_dir: &Option<PathBuf>,
    verbose: &bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = local_coq_conversion(original_llbc, refactored_llbc, out_dir)?;
    Ok(())
}