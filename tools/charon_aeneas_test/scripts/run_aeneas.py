import os
import subprocess

# Define directories
charon_dir = "/home/matt/rem-matt/rem-cli/tools/charon_aeneas_test/charon"  # Directory containing the files produced by charon.
aeneas_dir = "/home/matt/rem-matt/rem-cli/tools/charon_aeneas_test/aeneas"  # Destination directory for aeneas output.

# Create the aeneas output directory if it doesn't exist.
os.makedirs(aeneas_dir, exist_ok=True)

# Loop through each file in the charon directory.
for filename in os.listdir(charon_dir):
    file_path = os.path.join(charon_dir, filename)

    # Only process files (skip subdirectories, if any)
    if not os.path.isfile(file_path):
        continue

    print(f"Running aeneas on: {file_path}")

    # Construct the aeneas command:
    # -backend coq : tells aeneas to use the Coq backend.
    # -dest aeneas  : sets the destination directory for output.
    # file_path     : the current file from the charon directory.
    cmd = [
        "./aeneas",       # Adjust the command if you need a relative path (e.g., "./aeneas").
        "-backend", "coq",
        "-dest", aeneas_dir,
        file_path
    ]

    try:
        # Run the aeneas command.
        result = subprocess.run(
            cmd,
            capture_output=True,  # Capture stdout and stderr.
            text=True,            # Decode output as text.
            check=True            # Raise an error if the command fails.
        )
        print(f"Aeneas output for {filename}:\n{result.stdout}\n")
    except subprocess.CalledProcessError as e:
        print(f"Error processing {filename}:\n{e.stderr}")
