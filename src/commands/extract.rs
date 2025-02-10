use std::path::PathBuf;
use crate::refactor::extract_function::local_extract_method;
use crate::refactor::throughput::Extract;
use log::{
    info,
    error
};

pub fn extract(
    file_path: &PathBuf,
    new_fn_name: &String,
    start_index: &usize,
    end_index: &usize,
    verbose: &bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if *verbose {
        info!("File Path: {:?}", file_path);
        info!("New Function Name: {}", new_fn_name);
        info!("Start Index: {}", start_index);
        info!("End Index: {}", end_index);
        info!("Verbose Mode: {}", if *verbose { "yes" } else { "no" });
    } else {
        info!("Verbose Mode: {}", if *verbose { "yes" } else { "no" });
    }

    let start_idx: u32 = *start_index as u32;
    let end_idx: u32 = *end_index as u32;

    let input_extract: Extract = Extract::new(
        file_path.to_path_buf(),
        None, // output code not yet populated
        new_fn_name.to_string(),
        start_idx,
        end_idx,
        None, // caller_fn_name not yet populated
    );

    let result = local_extract_method(input_extract);
    // Unwrap the result, if success, log the new code and the
    // caller_fn_name
    let success: bool = match result {
        Ok(result) => {
            let output_code: String = result.get_output_code().unwrap();
            let caller_fn_name: String = result.get_caller_fn_name().unwrap();
            info!("New Code: \n{}", output_code);
            info!("Caller Function Name: {}", caller_fn_name);
            true
        }
        Err(e) => {
            error!("Extraction failed: {:?}", e);
            false
        }
    };

    Ok(())
}