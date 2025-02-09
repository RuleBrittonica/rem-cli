import os
import shutil

# Define the source and target directories
source_dir = "/home/matt/rem-matt/rem-cli/tools/charon_aeneas_test/correct_output"
target_dir = "/home/matt/rem-matt/rem-cli/tools/charon_aeneas_test/rust"

# Create the target directory if it does not exist
os.makedirs(target_dir, exist_ok=True)

# Loop through all files in the source directory
for filename in os.listdir(source_dir):
    # Only process files that end with .rs
    if filename.endswith(".rs"):
        # Split the filename into the base and extension
        base, ext = os.path.splitext(filename)
        # Create the new filename by appending _ref before the extension
        new_filename = f"{base}_ref{ext}"

        # Define the full path for the source and target files
        src_file = os.path.join(source_dir, filename)
        dst_file = os.path.join(target_dir, new_filename)

        # Copy the file to the target directory with the new name
        shutil.copy(src_file, dst_file)
        print(f"Copied '{src_file}' to '{dst_file}'")
