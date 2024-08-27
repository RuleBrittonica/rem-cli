# rem-cli
CLI for the REM Toolchain. Implemented in the VSCode extension for REM available at
[REM VSCode](https://marketplace.visualstudio.com/items?itemName=MatthewBritton.remvscode&ssr=false#overview)

**Utilizes**:

- rem-controller: git= [rem-controller](https://github.com/RuleBrittonica/rem-controller)
- rem-borrower: git= [rem-borrower](https://github.com/RuleBrittonica/rem-borrower)
- rem-repairer: git= [rem-repairer](https://github.com/RuleBrittonica/rem-repairer)
- rem-utils: git= [rem-utils](https://github.com/RuleBrittonica/rem-utils)

## Usage

Call the CLI using the following syntax

```bash
rem-cli [OPTIONS] [file_path] [new_file_path] [caller_fn_name] [callee_fn_name]
```

### Arguments
```bash
[file_path]       The path to the file that contains just the code that will be refactored
[new_file_path]   The path to the output file (where the refactored code ends up)
[caller_fn_name]  The name of the function that contains the code to be refactored
[callee_fn_name]  The name of the new function that is being extracted
```

### Options

```bash
  -t, --type <type>                     The type of refactoring - see README to learn what is currently supported. Leaving blank will run original REM extraction
  -T, --test                            Run the tests instead of refactoring. Ignores all other arguments
  -c, --controller                      Run the Controller on the input. Can be chained with borrower and repairer by adding their flags. Not specifying a flag is equivalent to -c -b -r
  -b, --borrower <borrower> <borrower>  Run the borrower on the input. Can be chaned with controller and repairer by adding their flags. Requires two additional arguments: `pre_extract_file_path` and `method_call_mut_file_path`.
  -r, --repairer <repairer>             Run the repairer on the input. Can be chained with controller and borrower by adding their flags. Requires the additional argument `repair_system`.
                                                 1 => repair_lifetime_simple
                                                 2 => tightest_bound_first
                                                 3 => loosest_bound_first
  -h, --help                            Print help
  -V, --version                         Print version
```

## Examples

**Running everything**

```bash
cargo run ./examples/input/full_1.rs ./examples/output/full_1.rs new_foo bar
```

**Running just the controller**

```bash
cargo run ./examples/input/controller_1.rs ./examples/output/controller_1.rs new_foo bar -c
```

**Running just the borrower**

```bash
cargo run ./src_tests/borrower/input/borrow_read_use_after.rs ./src_tests/borrower/output/borrow_read_use_after.rs new_foo bar -b src_tests/borrower/method_call_mut/borrow_read_use_after.rs src_tests/borrower/pre_extract/borrow_read_use_after.rs
```

**Running just the repairer**

```bash
cargo run ./examples/input/repairer_1.rs ./examples/output/repairer_1 -r
```

**Chaining multiple segments together**

```bash
cargo run ./examples/input/controller_1.rs ./examples/output/controller_borrower_1.rs new_foo bar -c -r
```

**Alternatively**

The program can be called using `./rem-cli`

```bash
./rem-cli ./examples/input/full_1.rs ./examples/output/full_1.rs new_foo bar
```

**Viewing help / version information**

Run these if you wish to see the above information on Options and Arguments
displayed in the terminal.

```bash
cargo run -- -h
cargo run -- -help
cargo run -- -V
cargo run -- --version
```

## Tests

The CLI integrates all tests written for the original REM toolchain into a
single command.

Running either the following:

* cargo run -- -T
* cargo run -- --test

Will result in running all of the tests for Controller, Borrower and Repairer.
At this stage, not all of the tests pass. This is more of a developer function,
however, the goal is that the user will also be able to run the test suite from
inside the extension in the event that they want to verify their environment.

Note that the testing framework will create a large number of temporary files
within the current directory. These will all be cleaned up at the end of each
testing phase.