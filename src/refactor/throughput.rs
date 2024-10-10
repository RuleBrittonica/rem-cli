use ra_ap_vfs::AbsPathBuf;

use rem_repairer::{
    common::RepairSystem,
    repair_lifetime_simple,
    repair_rustfix,
    repair_lifetime_loosest_bound_first,
    repair_lifetime_tightest_bound_first,
};

use crate::{
    utils::convert_to_abs_path_buf
};

/// This module is responsible for managing the inputs and outputs from the
/// various modules in the refactoring process.
/// At the moment it is a bit of a hack - the entire process needs to be redone
/// so that it only works on Strings of text instead of creating a ton of files
/// TODO This will require substantial changes to the borrower, controller and
/// TODO repairer.
/// This logic however, is implemented on the basis that the borrower, controller
/// and repairer take strings as input and output.
/// Some of the strings are populated when the struct is created, and some are
/// populated after the module has been run. In this case, the string is
/// optional

/// All of the data the user provides to the CLI to begin the refactoring
/// process
/// - file_path: The path to the file that will be refactored
/// - new_file_path: The path to the new file that will be created (optional, if
///   not provided, the original file will be overwritten)
/// - new_fn_name: The name of the new function that will be created
/// - start_idx: The number of characters to the start of the selection range
/// - end_idx: The number of characters to the end of the selection range
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Input {
    file_path: String,
    new_file_path: Option<String>,
    new_fn_name: String,
    start_idx: u32,
    end_idx: u32,
}

/// The throughput struct is the main data carrier for the refactoring process.
/// It contains an (immutable) version of the input code, and mutable versions
/// of the input and output code. It also contains references to all of the
/// other data needed throughout the process
/// To ensure compliance with all modules, all file paths are AbsPaths
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Throughput {
    file_path: AbsPathBuf,
    new_file_path: AbsPathBuf, // Populated with the original file path if not provided
    caller_fn_name: Option<String>, // The name of the function that contains the start and end idx
    new_fn_name: String,
    start_idx: u32,
    end_idx: u32,
    original_code: String, // Never changes, always stores a copy of the contained in the input file
    working_code: String, // The code that is being worked on (changes as the process progresses)
    temporary_code: Option<String>,
    output_code: Option<String>,
}

/// Data needed by the extraction module
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Extract {
    input_code: String,
    output_code: Option<String>, // Populated after the module has been run
    new_fn_name: String,
    start_idx: u32,
    end_idx: u32,
    caller_fn_name: Option<String>, // Populated after the module has been run
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Controller {
    input_code: String,
    output_code: Option<String>, // Populated after the module has been run
    caller_fn_name: String,
    new_fn_name: String,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Borrower {
    input_code: String,
    unmodified_code: String,
    output_code: Option<String>, // Populated after the module has been run
    caller_fn_name: String,
    new_fn_name: String,
    temporary_code: Option<String>, // Populated after the module has been run (for the mut_method_call_expr)
}

#[derive(Clone)]
pub struct Repairer<'a> {
    input_code: String,
    output_code: Option<String>, // Populated after the module has been run
    new_fn_name: String,
    repair_systems: Vec<&'a dyn RepairSystem>, // The repair systems that will be attempted to use to fix the code
}

impl Input {
    pub fn new(file_path: String, new_file_path: Option<String>, new_fn_name: String, start_idx: u32, end_idx: u32) -> Self {
        Self {
            file_path,
            new_file_path,
            new_fn_name,
            start_idx,
            end_idx,
        }
    }
}

impl Throughput {
    pub fn new(file_path: AbsPathBuf, new_file_path: AbsPathBuf, caller_fn_name: Option<String>, new_fn_name: String, start_idx: u32, end_idx: u32, original_code: String, working_code: String, temporary_code: Option<String>, output_code: Option<String>) -> Self {
        Self {
            file_path,
            new_file_path,
            caller_fn_name,
            new_fn_name,
            start_idx,
            end_idx,
            original_code,
            working_code,
            temporary_code,
            output_code,
        }
    }
}

impl Extract {
    pub fn new(input_code: String, output_code: Option<String>, new_fn_name: String, start_idx: u32, end_idx: u32, caller_fn_name: Option<String>) -> Self {
        Self {
            input_code,
            output_code,
            new_fn_name,
            start_idx,
            end_idx,
            caller_fn_name,
        }
    }
}

impl Controller {
    pub fn new(input_code: String, output_code: Option<String>, caller_fn_name: String, new_fn_name: String) -> Self {
        Self {
            input_code,
            output_code,
            caller_fn_name,
            new_fn_name,
        }
    }
}

impl Borrower {
    pub fn new(input_code: String, unmodified_code: String, output_code: Option<String>, caller_fn_name: String, new_fn_name: String, temporary_code: Option<String>) -> Self {
        Self {
            input_code,
            unmodified_code,
            output_code,
            caller_fn_name,
            new_fn_name,
            temporary_code,
        }
    }
}

impl Repairer<'_> {
    pub fn new(input_code: String, output_code: Option<String>, new_fn_name: String, repair_systems: Vec<&'static dyn RepairSystem>) -> Self {
        Self {
            input_code,
            output_code,
            new_fn_name,
            repair_systems,
        }
    }

    pub fn add_simple_repsys(&mut self) {
        self.repair_systems.push(
            &repair_lifetime_simple::Repairer {}
        );
    }

    pub fn add_rustfix_repsys(&mut self) {
        self.repair_systems.push(
            &repair_rustfix::Repairer {}
        );
    }

    pub fn add_loosest_bound_first_repsys(&mut self) {
        self.repair_systems.push(
            &repair_lifetime_loosest_bound_first::Repairer {}
        );
    }

    pub fn add_tightest_bound_first_repsys(&mut self) {
        self.repair_systems.push(
            &repair_lifetime_tightest_bound_first::Repairer {}
        );
    }

    /// Add all repair systems not currently in the repairer
    pub fn add_all_repsys(&mut self) {
        self.repair_systems = vec![
            &repair_lifetime_simple::Repairer {},
            &repair_rustfix::Repairer {},
            &repair_lifetime_loosest_bound_first::Repairer {},
            &repair_lifetime_tightest_bound_first::Repairer {},
        ];
    }
}


// =============================================================================
//                Implementing the From trait for the various structs
// =============================================================================


impl From<Input> for Throughput {
    /// Populate the new file path if it is not provided
    /// Create the original code from the file path
    /// Create the working code from the original code (at this stage they are the same)
    /// All optional fields are None (as they will be populated later but at
    /// this stage need to be checked as None)
    ///
    /// It is very important that this method is only used once on the input
    /// data. Otherwise the original code will be overwritten etc.
    fn from(input: Input) -> Self {

        let original_code_: String = std::fs::read_to_string( &input.file_path ).unwrap();

        Self {
            file_path: convert_to_abs_path_buf( &input.file_path ).unwrap(),
            new_file_path: convert_to_abs_path_buf( &input.new_file_path.unwrap_or( input.file_path )).unwrap(),
            caller_fn_name: None,
            new_fn_name: input.new_fn_name,
            start_idx: input.start_idx,
            end_idx: input.end_idx,
            original_code: original_code_.clone(),
            working_code: original_code_,
            temporary_code: None,
            output_code: None,
        }
    }
}

impl From<Throughput> for Extract {
    /// Pass the required data through to the extraction module
    /// `caller_fn_name` initialised as None - it will be populated after the
    /// extraction module has been run.
    fn from(throughput: Throughput) -> Self {
        Self {
            input_code: throughput.working_code,
            output_code: None,
            new_fn_name: throughput.new_fn_name,
            start_idx: throughput.start_idx,
            end_idx: throughput.end_idx,
            caller_fn_name: None,
        }
    }
}

impl From<Throughput> for Controller {
    fn from(throughput: Throughput) -> Self {
        Self {
            input_code: throughput.working_code,
            output_code: None,
            caller_fn_name: throughput.caller_fn_name.unwrap(),
            new_fn_name: throughput.new_fn_name,
        }
    }
}

impl From<Throughput> for Borrower {
    fn from(throughput: Throughput) -> Self {
        Self {
            input_code: throughput.working_code,
            unmodified_code: throughput.original_code,
            output_code: None,
            caller_fn_name: throughput.caller_fn_name.unwrap(),
            new_fn_name: throughput.new_fn_name,
            temporary_code: None,
        }
    }
}

impl From<Throughput> for Repairer<'_> {
    /// The relevant repair systems will need to be added manually after the
    /// from call. This is to maintain flexibility.
    fn from(throughput: Throughput) -> Self {
        Self {
            input_code: throughput.working_code,
            output_code: None,
            new_fn_name: throughput.new_fn_name,
            repair_systems: Vec::new(),
        }
    }
}