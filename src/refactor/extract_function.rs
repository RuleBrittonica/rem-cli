use std::path::PathBuf;

use rem_extract::extraction::{extract_method, ExtractionInput};
use crate::refactor::throughput::Extract;

pub fn local_extract_method(
    extraction_input: Extract,
) -> Result<Extract, Box<dyn std::error::Error>> {
    let mut extraction_result: Extract = extraction_input.clone();
    let extraction_input: ExtractionInput = extraction_input.into();
    let (output_code, caller_fn_name) = extract_method(extraction_input)
        .map_err(|e| format!("Failed to extract method: {}", e))?;

    // Chain the modifications and assign the result
    extraction_result = extraction_result
        .set_output_code(output_code)
        .set_caller_fn_name(caller_fn_name);

    // Return the updated extraction_result
    Ok(extraction_result)
}

pub fn test(path: PathBuf) -> Result<u8, Box<dyn std::error::Error>> {
    todo!("Implement tests for local_extract_method");
}