# rem-cli
CLI for the REM Toolchain. Implemented in the VSCode extension for REM available at
[REM VSCode](https://marketplace.visualstudio.com/items?itemName=MatthewBritton.remvscode&ssr=false#overview)


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