use rem_borrower::borrow;
use std::time::Instant;
use log::{
    info,
    error
};

use super::throughput::Borrower;
use crate::error::RefactoringError;

/// Only takes a file path as the non_local_controller has already setup the
/// final output filepath.
pub fn borrow(
    input: Borrower
) -> Result<String, RefactoringError> {
    todo!()
}