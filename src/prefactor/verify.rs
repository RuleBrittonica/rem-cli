//! Local hooks to the conversion and verification processes of
//! rem-verification.

use std::path::PathBuf;

use rem_verification::{
    convert::coq_conversion,
    verify::coq_verification,
};

/// Returns the paths to the original and new .v (CoQ) files.
pub fn local_coq_conversion(
    original_llbc: &PathBuf,
    refactored_llbc: &PathBuf,
    out_dir: &Option<PathBuf>,
) -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    coq_conversion(original_llbc, refactored_llbc, out_dir)
}

/// Returns the paths to:
/// - The _CoQProject file
/// - The EquivCheck.v file
/// - The Primitives.v file
pub fn local_coq_verification(
    original_coq: &PathBuf,
    refactored_coq: &PathBuf,
    top_level_function: &String,
) -> Result<( PathBuf, PathBuf, PathBuf, bool ), Box<dyn std::error::Error>> {
    coq_verification( original_coq, refactored_coq, top_level_function )
}