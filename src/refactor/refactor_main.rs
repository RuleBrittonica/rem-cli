use crate::error::RefactoringError;

use crate::refactor::throughput::Input;

/// Calls out to rem-extract, rem-controller, then rem-borrower, then rem-repairer to fix up
/// the extracted method.
///
/// Sequentially calls the nlc, borrower and repairer defined in this module as
/// well. Will iterate through each repair method until one works.
pub fn extract_function(
    input: Input
) -> Result<String, RefactoringError> {
    // Change this to return a result.
    todo!()
}

pub fn extract_function_generic(
    input: Input,
) -> Result<String, RefactoringError> {
    todo!()
}

pub fn extract_function_async(
    input: Input,
) -> Result<String, RefactoringError> {
    todo!()
}
