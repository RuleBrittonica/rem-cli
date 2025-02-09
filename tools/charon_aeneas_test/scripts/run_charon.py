import os
import subprocess

# Directories
rust_dir = "/home/matt/rem-matt/rem-cli/tools/charon_aeneas_test/rust"     # Folder containing the input .rs files.
charon_dir = "/home/matt/rem-matt/rem-cli/tools/charon_aeneas_test/charon" # Folder where charon will output its files.

# Create the output directory if it does not exist.
os.makedirs(charon_dir, exist_ok=True)

# List all files in the rust directory.
for filename in os.listdir(rust_dir):
    # Process only .rs files.
    if filename.endswith(".rs"):
        # Full path to the input file.
        input_file = os.path.join(rust_dir, filename)
        print(f"Running charon on: {input_file}")

        # Create the destination file. The dest file must have the .llbc
        # extension.
        dest_file = os.path.join(charon_dir, f"{filename}.llbc")

        # Construct the command.
        # This calls charon with:
        #   --hide-marker-traits flag set
        #   --dest set to the charon directory
        #   --input specifying the current file from rust_dir.
        cmd = [
            "./charon",               # Replace with "./charon" if needed.
            "--hide-marker-traits",
            "--dest-file", dest_file,
            "--input", input_file
        ]

        try:
            # Run charon and capture its output.
            result = subprocess.run(
                cmd,
                capture_output=True,  # Captures stdout and stderr.
                text=True,            # Decode output as text.
                check=True            # Raises an exception if the command fails.
            )
            print(f"CHARON output for {filename}:\n{result.stdout}\n")
        except subprocess.CalledProcessError as e:
            print(f"Error processing {filename}:\n{e.stderr}")
