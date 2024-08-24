use crate::error::ExtractFnBodyError;

// use crate::prefactor::lsp_call::lsp_main;

/// Extracts the function body into a new function
/// If the `file_path` and `new_file_path` are the same, the the original file
/// will be overwritten
///
/// # Args:
/// * `file_path` - The path to the input file
/// * `new_file_path` - The path the the output file
/// * `calle_fn_name` - The name of the function that contains the two cursor references ($0 -> $0)
/// * `caller_fn_name` - The name of the function to be created
///
/// # Returns:
/// * Result of either nothing (success) or an Extract_FN_Body_Error
pub fn extract_fn_body(
    file_path: &str,
    new_file_path: &str,
    calle_fn_name: &str,
    caller_fn_name: &str,
) -> Result<(), ExtractFnBodyError> {

    if file_path.is_empty() {
        return Err(ExtractFnBodyError::FnNotFoundError(calle_fn_name.to_string()));
    }

    // let res = lsp_main();
    // match res {
    //     Ok(_) => println!("LSP Success"),
    //     Err(e) => println!("LSP Errored: {}", e)
    // }

    Ok(())
}