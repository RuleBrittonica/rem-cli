# rem-cli

CLI for the REM Toolchain. Implemented in the VSCode extension for REM available at
[REM VSCode](https://marketplace.visualstudio.com/items?itemName=MatthewBritton.remvscode&ssr=false#overview)

**Utilizes**:

- rem-controller: git= [rem-controller](https://github.com/RuleBrittonica/rem-controller)
- rem-borrower: git= [rem-borrower](https://github.com/RuleBrittonica/rem-borrower)
- rem-repairer: git= [rem-repairer](https://github.com/RuleBrittonica/rem-repairer)

## Getting Started

Make sure that you have the developer tools for rustc installed on your system.
Run the following command if you are unsure. This toolchain is built for
**nightly-2024-08-28**. Others may work but are not tested.

```bash
rustup component add --toolchain nightly-2024-08-28 rust-src rustc-dev llvm-tools-preview
```

Additionally, at some point in the future this CLI may also be dependent on
`rust-analyzer`. Probably best to double check it as I'm sure I'll forget to
update this when it becomes dependent on RLS.

```bash
rustup component add rust-analyzer
```

From there, `rust-toolchain.toml` should be able to do the rest of the heavy
lifting. Refer to its components list if you are unsure.

## Usage

Call the CLI using the following syntax

```bash
cargo run <COMMAND>
```

or

```bash
./rem-cli-lsp <COMMNAD>
```

## Commands

**run**

Arguments:

```bash
<FILE_PATH>       The path to the file that contains just the code that will be refactored
<NEW_FILE_PATH>   The path to the output file (where the refactored code ends up)
<CALLER_FN_NAME>  The name of the function that contains the code to be refactored
<CALLEE_FN_NAME>  The name of the new function that is being extracted
```

Running:

```bash

```

**controller**

Arguments:

```bash
<FILE_PATH>       The path to the file that contains just the code that will be refactored
<NEW_FILE_PATH>   The path to the output file (where the refactored code ends up)
<CALLER_FN_NAME>  The name of the function that contains the code to be refactored
<CALLEE_FN_NAME>  The name of the new function that is being extracted
```

Running:

```bash

```

**borrower**

Arguments:

```bash
<FILE_PATH>       The path to the file that contains just the code that will be refactored
<NEW_FILE_PATH>   The path to the output file (where the refactored code ends up)
<CALLER_FN_NAME>  The name of the function that contains the code to be refactored
<CALLEE_FN_NAME>  The name of the new function that is being extracted
```

Running:

```bash

```

**repairer**

Arguments:

```bash
<FILE_PATH>      The path to the file that contains just the code that will be refactored
<NEW_FILE_PATH>  The path to the output file (where the refactored code ends up)
<FN_NAME>        The name of the function to be repaired
<REPAIRER>       Repairer option (1=Simple, 2=Loosest Bounds First, 3=Tightest Bounds First)
```

Running:

```bash

```

**repairer-cargo**

Arguments:

```bash
<SRC_PATH>
<MANIFEST_PATH>
<FN_NAME>
<REPAIRER>       Repairer option (1=Simple, 2=Loosest Bounds First, 3=Tightest Bounds First)
```

Running:

```bash

```

**test**

Runs the test suite, against the specified filepath. The test suite must contain
the following subdirs

- borrower
- controller
- repairer

Arguments:

```bash
<Folder> Folder path containing test files
```

Running:

```bash
cargo run test src_tests/
```

**test-github**

This command is the same as running test, however, you specify a link to a
github repo that contains the files you want to test on,

Arguments:

```bash
<REPO>  Folder path containing test files
```

```bash
cargo run test-github https://github.com/RuleBrittonica/rem-testfiles
```

### Options

```bash
  -h, --help                            Print help
  -V, --version                         Print version
```

## Examples

**Running everything**

```bash
cargo run run ./examples/input/full_1.rs ./examples/output/full_1.rs new_foo bar
```

**Running just the controller**

```bash
cargo run controller ./examples/input/controller_1.rs ./examples/output/controller_1.rs new_foo bar
```

**Running just the borrower**

```bash

```

**Running just the repairer**

For the repairer, only the callee_fn_name is used, however, both must still be
provided to get the CLI to accept the argument.

```bash
# 1 signifies the mode that we are running the repairer in - see above documentation for different repairer modes.
```

By default the repairer will run in mode 1 (repair_lifetime_simple). To specify
which version of the repairer to run when running in general, refer to the
following:

```bash

```

**Chaining multiple segments together**

This is currently very buggy and not recommended to do.

```bash

```

**Alternatively**

The program can be called using `./rem-cli`

```bash

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

```bash
cargo run -- -T
cargo run -- --test
```

Will result in running all of the tests for Controller, Borrower and Repairer.
At this stage, **not all of the tests pass**. This is more of a developer function,
however, the goal is that the user will also be able to run the test suite from
inside the extension in the event that they want to verify their environment.

Note that the testing framework will create a large number of temporary files
within the current directory. These will all be cleaned up at the end of each
testing phase.

## TODO

- Work out why I keep getting a panic whenever the tests get halfway through
  running the borrower. At this stage a workable solution is just to comment
  out the running of the borrower tests to verify that the repairer works as expected

```bash
   thread 'main' panicked at /home/matt/.cargo/git/checkouts/rem-borrower-c9dc79a7e6c71e4e/d760805/src/borrow.rs:1104:10:
   called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
   note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- Verify that all aspects of the CLI work as expected
- Fix up the issues with running rem-cli directly

```bash
  target/debug/rem-cli: error while loading shared libraries: librustc_driver-6c98eb7349a51df2.so: cannot open shared object file: No such file or directory
```

- **The big one** Add integration to RLS (or do it on the VSCode side potentially?)
- Update all package references to use crates instead of github, once I have the
  access from Sewen. Start with rem-utils, then link everything into that
  instead. This should hopefully fix the `./rem-cli` issues I am having.
- Implement the controller, borrower and repairer. Both the CLI end, and the
  actual functions, need to be implemented
- Implement the complete refactoring toolchain (i.e. give file and context, and
  refactoring happens from there)
- Update the documentation.
