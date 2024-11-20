# rem-cli

CLI for the REM Toolchain. Implemented in the VSCode extension for REM available at
[REM VSCode](https://marketplace.visualstudio.com/items?itemName=MatthewBritton.remvscode&ssr=false#overview)

**Utilizes**:

-rem-extract:
  - git = [rem-extract](https://github.com/RuleBrittonica/rem-extract)
  - crates = [rem-extract](https://crates.io/crates/rem-extract)
- rem-controller:
  - git = [rem-controller](https://github.com/RuleBrittonica/rem-controller)
  - crates = [rem-controller](https://crates.io/crates/rem-controller)
- rem-borrower:
  - git = [rem-borrower](https://github.com/RuleBrittonica/rem-borrower)
  - crates = [rem-borrower](https://crates.io/crates/rem-borrower)
- rem-repairer:
  - git = [rem-repairer](https://github.com/RuleBrittonica/rem-repairer)
  - crates = [rem-repairer](https://crates.io/crates/rem-repairer)

## Getting Started

Make sure that you have the developer tools for rustc installed on your system.
Run the following command if you are unsure. This toolchain is built for
**nightly-2024-08-28**. Others may work but are not tested.

```bash
rustup toolchain install nightly-2024-08-28
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
./rem-cli <COMMAND>
```

## Commands

Currently implemented commands:

- controller
- borrower
- repairer
- test
- test-github

### run (NYI)

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

### controller

Arguments:

```bash
<FILE_PATH>       The path to the file that contains just the code that will be refactored
<NEW_FILE_PATH>   The path to the output file (where the refactored code ends up)
<CALLER_FN_NAME>  The name of the function that contains the code to be refactored
<CALLEE_FN_NAME>  The name of the new function that is being extracted
```

Running:

```bash
cargo run controller examples/input/controller_1.rs examples/output/controller_1.rs new_foo bar
```

### borrower

Arguments:

```bash
<FILE_PATH>              The path to the file that contains just the code that will be refactored
<NEW_FILE_PATH>          The path to the output file (where the refactored code ends up)
<CALLER_FN_NAME>         The name of the function that contains the code to be refactored
<CALLEE_FN_NAME>         The name of the new function that is being extracted
<MUT_METHOD_FILE_PATH>   The path to the file where the mutated method will be dumped
<PRE_EXTRACT_FILE_PATH>  The path to the original file
```

Running:

```bash
cargo run borrower examples/input/borrow_1.rs example/output/borrow_1.rs new_foo bar examples/mcm/borrow_1.rs examples/pe/borrow_1.rs
```

### repairer

Arguments:

```bash
<FILE_PATH>      The path to the file that contains just the code that will be refactored
<NEW_FILE_PATH>  The path to the output file (where the refactored code ends up)
<FN_NAME>        The name of the function to be repaired
<REPAIRER>       Repairer option (1=Simple, 2=Loosest Bounds First, 3=Tightest Bounds First, 4=Rustfix)
```

Running:

```bash
cargo run repairer examples/input/repair_1.rs examples/output/repair_1.rs bar_extracted 1 # Use Simple Repair mechanism
```

### repairer-cargo (NYI)

Arguments:

```bash
<SRC_PATH>
<MANIFEST_PATH>
<FN_NAME>
<REPAIRER>       Repairer option (1=Simple, 2=Loosest Bounds First, 3=Tightest Bounds First, 4=Rustfix)
```

Running:

```bash

```

### test

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

At this stage, **NOT ALL OF THE TESTS PASS**. This is more of a developer function,
however, the goal is that the user will also be able to run the test suite from
inside the extension in the event that they want to verify their environment.

Note that the testing framework will create a large number of temporary files
within the current directory. These will all be cleaned up at the end of each
testing phase.

### test-github

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

## Getting started with fresh linux install

First install rust, proceeding with the standard options:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then install the nescessary toolchain components:

```bash
rustup toolchain install nightly-2024-08-28
rustup component add --toolchain nightly-2024-08-28 rust-src rustc-dev llvm-tools-preview
rustup component add rust-analyzer
```

Then install the nescessary build components

```bash
sudo apt update
sudo apt install build-essential
```

Finally install the openssl dependencies

```bash
sudo apt install pkg-config libssl-dev
```

## TODO

- **The big one** Add integration to RLS (or do it on the VSCode side potentially?)
- Implement the controller, borrower and repairer. Both the CLI end, and the
  actual functions, need to be implemented
- Implement the complete refactoring toolchain (i.e. give file and context, and
  refactoring happens from there)
- Update the documentation.
