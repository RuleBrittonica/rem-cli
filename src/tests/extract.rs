use rem_extract::{
    extraction::{
        ExtractionInput,
        extract_method,
    },
    error::ExtractionError,
    };
use crate::tests::utils::compile_file;
use log::{
    error,
    info,
};

use colored::Colorize;
use std::{
    fs::{self, read_to_string},
    io,
    path::{
        Path,
        PathBuf,
    },
    time::{
        SystemTime,
        Instant,
        Duration,
    }
};

use rem_utils::{
    remove_all_files,
    strip_ansi_codes,
    parse_and_compare_ast, // Different to rem-extract's parse_and_compare_ast. It takes two strings and compares the ASTs, returning a bool
    print_file_diff
};

use lazy_static::lazy_static;

use crate::refactor::throughput::Extract;

struct TestFile<'a> {
    pub input_file: &'a str, // Just the name of the file. It is assumed the file is in ./input/{testname}/src/main.rs, and there is a corresponding file in ./correct_output (./correct_output/{testname}.rs)
    pub start_idx: u32,
    pub end_idx: u32,
}

impl TestFile<'_> {
    pub fn new(input_file: &str, start_idx: u32, end_idx: u32) -> TestFile {
        TestFile {
            input_file,
            start_idx,
            end_idx,
        }
    }
}

/// A TestInput needs a Path to write to, as part of the testing process is
/// writing the output to a file. This is not needed for the actual extraction
struct TestInput{
    pub file_path: String,
    pub output_path: String,
    pub new_fn_name: String,
    pub start_idx: u32,
    pub end_idx: u32,
}

/// Helper trait to convert a TestFile into an ExtractionInput
impl From<&TestFile<'_>> for TestInput {
    fn from(test_file: &TestFile<'_>) -> TestInput {
        let file_path: String = PathBuf::from("./src_tests/extract/input")
            .join(&test_file.input_file)
            .join("src")
            .join("main.rs")
            .to_string_lossy()
            .to_string();

        let output_path: String = PathBuf::from("./src_tests/extract/output")
            .join(&test_file.input_file)
            .with_extension("rs")
            .to_string_lossy()
            .to_string();

        TestInput {
            file_path,
            output_path,
            new_fn_name: "fun_name".to_string(),
            start_idx: test_file.start_idx,
            end_idx: test_file.end_idx,
        }
    }
}

impl From<&TestInput> for ExtractionInput {
    fn from(test_input: &TestInput) -> ExtractionInput {
        ExtractionInput {
            file_path: test_input.file_path.clone(),
            new_fn_name: test_input.new_fn_name.clone(),
            start_idx: test_input.start_idx,
            end_idx: test_input.end_idx,
        }
    }
}

/// Equivalent to the test_verbose method from rem-extract.
/// Takes in a path to a directory containing the following folders:
/// - input: Contains the input files to be refactored
/// - output: Where the refactored files will be written to
/// - correct_output: Contains the correct refactored files (used for comparison)
/// Returns a u8 representing the number of tests that failed (0 if all passed)
pub fn test(path: PathBuf) -> Result<u8, io::Error> {

    // Clear the output directory
    let output_dir = path.join("extract/output");
    remove_all_files(&output_dir);

    // Measure total time at the start
    let overall_start_time: Instant = Instant::now();

    info!("Running method extraction tests from directory {}", path.to_str().unwrap());

    // Initialize counters and time trackers
    let mut total_tests: i32 = 0;
    let mut passed_stage_1: i32 = 0;
    let mut passed_tests: i32 = 0;
    let mut failed_tests: i32 = 0;
    let mut total_test_time: Duration = Duration::new(0, 0);
    let mut min_test_time: Option<Duration> = None;
    let mut max_test_time: Option<Duration> = None;

    let allowed_tests: Vec<&'static str> = vec![
        // "break_loop_nested",
        // "comments_in_block_expr",
    ];

    for (index, test_file) in TEST_FILES.iter().enumerate() {
        let test_start_time: Instant = Instant::now();

        total_tests += 1;

        let input: TestInput = TestInput::from(test_file);
        let expected_file_path: String = PathBuf::new()
            .join("./src_tests/extract/correct_output")
            .join(&test_file.input_file)
            .with_extension("rs")
            .to_string_lossy()
            .to_string();
        let output_path: String = input.output_path.clone();

        let extraction_input: ExtractionInput = ExtractionInput::from(&input);

        // Call the extraction method and handle errors
        let extraction_result: Result<(String, _), ExtractionError> = extract_method(extraction_input);

        // Measure time taken for extraction
        let test_elapsed_time: Duration = test_start_time.elapsed();
        total_test_time += test_elapsed_time;

        // Update min and max times
        if let Some(min_time) = min_test_time {
            if test_elapsed_time < min_time {
                min_test_time = Some(test_elapsed_time);
            }
        } else {
            min_test_time = Some(test_elapsed_time);
        }

        if let Some(max_time) = max_test_time {
            if test_elapsed_time > max_time {
                max_test_time = Some(test_elapsed_time);
            }
        } else {
            max_test_time = Some(test_elapsed_time);
        }

        let test_elapsed_time_secs: f64 = test_elapsed_time.as_secs_f64();
        let test_elapsed_time_str: String = if test_elapsed_time_secs < 1.0 {
            format!("{:.2}ms", test_elapsed_time_secs * 1000.0)
        } else {
            format!("{:.2}s", test_elapsed_time_secs)
        };

        let test_name: &str = test_file.input_file.trim_end_matches(".rs");
        let mut extraction_status: String = "FAILED".red().to_string();
        let mut comparison_status: String = "N/A".to_string(); // Default to not applicable

        if extraction_result.is_ok() {
            // Unwrap the result to get the output code (as we know that it is
            // successful)
            // Also write the output code to the output file for later viewing
            let (extraction_result, _)  = extraction_result.unwrap();
            fs::write(&output_path, &extraction_result).unwrap();
            extraction_status = "PASSED".green().to_string();
            passed_stage_1 += 1;
            let expected_code = read_to_string(&expected_file_path).unwrap();

            // Compare the output file with the expected file's AST
            match parse_and_compare_ast(&extraction_result, &expected_code) {
                Ok(is_identical) => {
                    if is_identical {
                        comparison_status = "PASSED".green().to_string();
                        passed_tests += 1;
                    } else {
                        comparison_status = "FAILED".red().to_string();
                        failed_tests += 1;
                    }
                }
                Err(e) => {
                    comparison_status = format!("Error: {}", e).red().to_string();
                    failed_tests += 1;
                }
            }
        } else if let Err(e) = extraction_result {
            extraction_status = format!("FAILED: {}", e).red().to_string();
            failed_tests += 1;
        }

        println!("Test {} | {} | {}: {} in {}", index + 1, extraction_status, comparison_status, test_name, test_elapsed_time_str);
        // Strip ANSI color codes before logging
        let clean_extraction_status = strip_ansi_codes(&extraction_status);
        let clean_comparison_status = strip_ansi_codes(&comparison_status);

        info!("Test {} | {} | {}: {} in {}", index + 1, clean_extraction_status, clean_comparison_status, test_name, test_elapsed_time_str);

    }

    // Total elapsed time
    let total_elapsed_time: Duration = overall_start_time.elapsed();
    let total_elapsed_time_secs: f64 = total_elapsed_time.as_secs_f64();
    let total_elapsed_time_str: String = if total_elapsed_time_secs < 1.0 {
        format!("{:.2}ms", total_elapsed_time_secs * 1000.0)
    } else {
        format!("{:.2}s", total_elapsed_time_secs)
    };

    // Calculate average time per test
    let average_time_per_test: f64 = if total_tests > 0 {
        total_test_time.as_secs_f64() / total_tests as f64
    } else {
        0.0
    };

    let average_time_str: String = if average_time_per_test < 1.0 {
        format!("{:.2}ms", average_time_per_test * 1000.0)
    } else {
        format!("{:.2}s", average_time_per_test)
    };

    // Print overall statistics
    println!("------------------------------------------------------------------");
    println!("Total tests run: {}", total_tests);
    println!("Tests passed stage 1: {}", passed_stage_1);
    println!("Tests passed: {}", passed_tests);
    println!("Tests failed: {}", failed_tests);
    println!("Total time: {}", total_elapsed_time_str);
    println!("Average time per test: {}", average_time_str);

    // Log overall statistics
    info!("------------------------------------------------------------------");
    info!("Total tests run: {}", total_tests);
    info!("Tests passed stage 1: {}", passed_stage_1);
    info!("Tests passed: {}", passed_tests);
    info!("Tests failed: {}", failed_tests);
    info!("Total time: {}", total_elapsed_time_str);
    info!("Average time per test: {}", average_time_str);

    if let Some(min_time) = min_test_time {
        let min_time_secs: f64 = min_time.as_secs_f64();
        let min_time_str: String = if min_time_secs < 1.0 {
            format!("{:.2}ms", min_time_secs * 1000.0)
        } else {
            format!("{:.2}s", min_time_secs)
        };
        println!("Shortest test time: {}", min_time_str);
        info!("Shortest test time: {}", min_time_str);
    }

    if let Some(max_time) = max_test_time {
        let max_time_secs: f64 = max_time.as_secs_f64();
        let max_time_str: String = if max_time_secs < 1.0 {
            format!("{:.2}ms", max_time_secs * 1000.0)
        } else {
            format!("{:.2}s", max_time_secs)
        };
        println!("Longest test time: {}", max_time_str);
        info!("Longest test time: {}", max_time_str);
    }

    // Return the number of failed tests
    Ok(failed_tests as u8)

}


lazy_static! {
    static ref TEST_FILES: Vec<TestFile<'static>> = vec![
        TestFile::new(
            "argument_and_locals",
            39,
            60,
        ),
        TestFile::new(
            "argument_form_expr",
            39,
            42,
        ),
        TestFile::new(
            "argument_used_twice_form_expr",
            39,
            42,
        ),
        TestFile::new(
            "break_from_nested_and_outer_loops",
            52,
            186,
        ),
        TestFile::new(
            "break_from_nested_loop",
            52,
            133,
        ),
        TestFile::new(
            "break_loop",
            52,
            102,
        ),
        TestFile::new(
            "break_loop_nested",
            56,
            123,
        ),
        TestFile::new(
            "break_loop_nested_labeled",
            58,
            69,
        ),
        TestFile::new(
            "break_loop_with_if",
            56,
            103,
        ),
        TestFile::new(
            "break_stmt",
            46,
            57,
        ),
        TestFile::new(
            "break_with_value",
            59,
            148,
        ),
        TestFile::new(
            "break_with_value_and_label",
            65,
            159,
        ),
        TestFile::new(
            "break_with_value_and_return",
            49,
            148,
        ),
        TestFile::new(
            "closure_arguments",
            57,
            88,
        ),
        TestFile::new(
            "comments_in_block_expr",
            22,
            133,
        ),
        TestFile::new(
            "continue_loop_nested_labeled",
            58,
            72,
        ),
        TestFile::new(
            "copy_custom_used_after",
            87,
            99,
        ),
        TestFile::new(
            "copy_used_after",
            32,
            42,
        ),
        TestFile::new(
            "does_not_add_extra_whitespace",
            27,
            39,
        ),
        TestFile::new(
            "does_not_import_control_flow",
            17,
            44,
        ),
        TestFile::new(
            "dont_emit_type_with_hidden_lifetime_parameter",
            73,
            80,
        ),
        TestFile::new(
            "empty_generic_param_list",
            52,
            59,
        ),
        TestFile::new(
            "extract_cast",
            25,
            36,
        ),
        TestFile::new(
            "extract_does_not_tear_body_apart",
            16,
            25,
        ),
        TestFile::new(
            "extract_does_not_tear_comments_apart",
            18,
            52,
        ),
        TestFile::new(
            "extract_does_not_wrap_res_in_res",
            35,
            74,
        ),
        TestFile::new(
            "extract_from_nested",
            76,
            81,
        ),
        TestFile::new(
            "extract_function_copies_comment_at_end",
            33,
            65,
        ),
        TestFile::new(
            "extract_function_copies_comment_at_start",
            33,
            65,
        ),
        TestFile::new(
            "extract_function_copies_comment_in_between",
            27,
            81,
        ),
        TestFile::new(
            "extract_function_copies_comment_indented",
            33,
            95,
        ),
        TestFile::new(
            "extract_function_does_preserve_whitespace",
            33,
            61,
        ),
        TestFile::new(
            "extract_function_from_trait_with_existing_non_empty_impl_block",
            186,
            191,
        ),
        TestFile::new(
            "extract_function_long_form_comment",
            33,
            64,
        ),
        TestFile::new(
            "extract_method_from_trait_impl",
            129,
            139,
        ),
        TestFile::new(
            "extract_method_from_trait_with_existing_non_empty_impl_block",
            166,
            176,
        ),
        TestFile::new(
            "extract_method_from_trait_with_multiple_existing_impl_blocks",
            301,
            311,
        ),
        TestFile::new(
            "extract_method_from_trait_with_multiple_existing_trait_impl_blocks",
            441,
            451,
        ),
        TestFile::new(
            "extract_mut_ref_param_has_no_mut_binding_in_loop",
            140,
            148,
        ),
        TestFile::new(
            "extract_partial_block",
            62,
            86,
        ),
        TestFile::new(
            "extract_partial_block_single_line",
            44,
            50,
        ),
        TestFile::new(
            "extract_return_stmt",
            23,
            35,
        ),
        TestFile::new(
            "extract_with_await",
            17,
            39,
        ),
        TestFile::new(
            "extract_with_await_and_result_not_producing_match_expr",
            40,
            69,
        ),
        TestFile::new(
            "extract_with_await_and_result_producing_match_expr",
            55,
            170,
        ),
        TestFile::new(
            "extract_with_await_in_args",
            17,
            59,
        ),
        TestFile::new(
            "extract_with_macro_arg",
            23,
            31,
        ),
        TestFile::new(
            "filter_unused_generics",
            59,
            66,
        ),
        TestFile::new(
            "filter_unused_where_clause",
            70,
            77,
        ),
        TestFile::new(
            "filters_unused_nested_generics",
            181,
            200,
        ),
        TestFile::new(
            "filters_unused_nested_where_clauses",
            211,
            230,
        ),
        TestFile::new(
            "function_expr",
            16,
            26,
        ),
        TestFile::new(
            "indented_stmts",
            55,
            89,
        ),
        TestFile::new(
            "indented_stmts_inside_mod",
            82,
            120,
        ),
        TestFile::new(
            "it_should_not_generate_duplicate_function_names",
            21,
            31,
        ),
        TestFile::new(
            "method_to_freestanding",
            59,
            62,
        ),
        TestFile::new(
            "method_with_mut",
            67,
            79,
        ),
        TestFile::new(
            "method_with_reference",
            70,
            83,
        ),
        TestFile::new(
            "multi_variables_defined_inside_and_used_after_mutably_no_ret",
            32,
            111,
        ),
        TestFile::new(
            "mut_field_from_outer_scope",
            66,
            75,
        ),
        TestFile::new(
            "mut_method_call",
            133,
            141,
        ),
        TestFile::new(
            "mut_nested_field_from_outer_scope",
            165,
            205,
        ),
        TestFile::new(
            "mut_param_because_of_mut_ref",
            36,
            65,
        ),
        TestFile::new(
            "mut_param_many_usages_expr",
            236,
            420,
        ),
        TestFile::new(
            "mut_param_many_usages_stmt",
            236,
            370,
        ),
        TestFile::new(
            "mut_var_from_outer_scope",
            36,
            43,
        ),
        TestFile::new(
            "nested_generics",
            155,
            174,
        ),
        TestFile::new(
            "nested_where_clauses",
            179,
            198,
        ),
        TestFile::new(
            "no_args_for",
            16,
            36,
        ),
        TestFile::new(
            "no_args_from_binary_expr",
            20,
            25,
        ),
        TestFile::new(
            "no_args_from_binary_expr_in_module",
            39,
            44,
        ),
        TestFile::new(
            "no_args_from_binary_expr_indented",
            16,
            25,
        ),
        TestFile::new(
            "no_args_from_loop_unit",
            16,
            49,
        ),
        TestFile::new(
            "no_args_from_loop_with_return",
            24,
            75,
        ),
        TestFile::new(
            "no_args_from_match",
            29,
            94,
        ),
        TestFile::new(
            "no_args_from_stmt_unit",
            32,
            62,
        ),
        TestFile::new(
            "no_args_from_stmt_with_last_expr",
            39,
            60,
        ),
        TestFile::new(
            "no_args_if",
            16,
            27,
        ),
        TestFile::new(
            "no_args_if_else",
            23,
            47,
        ),
        TestFile::new(
            "no_args_if_let_else",
            23,
            59,
        ),
        TestFile::new(
            "no_args_match",
            23,
            83,
        ),
        TestFile::new(
            "no_args_while",
            16,
            30,
        ),
        TestFile::new(
            "non_tail_expr_of_tail_expr_loop",
            34,
            77,
        ),
        TestFile::new(
            "non_tail_expr_of_tail_if_block",
            55,
            132,
        ),
        TestFile::new(
            "non_tail_expr_with_comment_of_tail_expr_loop",
            34,
            99,
        ),
        TestFile::new(
            "nontrivial_patterns_define_variables",
            38,
            66,
        ),
        TestFile::new(
            "param_from_closure",
            39,
            44,
        ),
        TestFile::new(
            "param_usage_in_macro",
            23,
            41,
        ),
        TestFile::new(
            "param_usage_in_macro_with_nested_tt",
            23,
            51,
        ),
        TestFile::new(
            "param_usage_in_macro_with_nested_tt_2",
            23,
            56,
        ),
        TestFile::new(
            "part_of_expr_stmt",
            16,
            17,
        ),
        TestFile::new(
            "preserve_generics",
            31,
            38,
        ),
        TestFile::new(
            "preserve_generics_from_body",
            34,
            46,
        ),
        TestFile::new(
            "preserve_where_clause",
            39,
            46,
        ),
        TestFile::new(
            "reference_mutable_param_with_further_usages",
            76,
            90,
        ),
        TestFile::new(
            "reference_mutable_param_without_further_usages",
            76,
            90,
        ),
        TestFile::new(
            "return_from_nested_fn",
            52,
            139,
        ),
        TestFile::new(
            "return_from_nested_loop",
            42,
            134,
        ),
        TestFile::new(
            "return_to_parent",
            39,
            84,
        ),
        TestFile::new(
            "should_increment_suffix_until_it_finds_space",
            60,
            70,
        ),
        TestFile::new(
            "struct_with_two_fields_pattern_define_variables",
            52,
            101,
        ),
        TestFile::new(
            "tail_expr_no_extra_control_flow",
            39,
            92,
        ),
        TestFile::new(
            "tail_expr_of_tail_block_nested",
            47,
            196,
        ),
        TestFile::new(
            "try_and_return_ok",
            51,
            134,
        ),
        TestFile::new(
            "try_option",
            85,
            120,
        ),
        TestFile::new(
            "try_option_unit",
            46,
            81,
        ),
        TestFile::new(
            "try_option_with_return",
            46,
            128,
        ),
        TestFile::new(
            "try_result",
            51,
            86,
        ),
        TestFile::new(
            "try_result_with_return",
            51,
            135,
        ),
        TestFile::new(
            "two_arguments_form_expr",
            55,
            60,
        ),
        TestFile::new(
            "two_variables_defined_inside_and_used_after_no_ret",
            32,
            66,
        ),
        TestFile::new(
            "unresolveable_types_default_to_placeholder",
            51,
            54,
        ),
        TestFile::new(
            "variable_defined_inside_and_used_after_mutably_no_ret",
            32,
            50,
        ),
        TestFile::new(
            "variable_defined_inside_and_used_after_no_ret",
            32,
            46,
        ),
    ];
}