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

/// Replace the code in the file with the new (refactored) code
/// and return the path to the file
/// Is just a blanket replace of all the code in the original file with the new
/// code passed in.
/// TODO This is a naiive implementation and should be improved.
pub fn replace_text_in_file(
    file_path: &PathBuf,
    new_code: &String,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Write the new code to the file. This will truncate the file if it exists,
    // or create a new file if it doesn't.
    std::fs::write(file_path, new_code)?;

    // Return the path of the file that was updated.
    Ok(file_path.clone())
}

pub fn test(path: PathBuf) -> Result<u8, Box<dyn std::error::Error>> {
    todo!("Implement tests for local_extract_method");
}