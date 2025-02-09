import os
import shutil

# Define the input directory (containing crates) and target directory
input_dir = "/home/matt/rem-matt/rem-cli/tools/charon_aeneas_test/input"
target_dir = "/home/matt/rem-matt/rem-cli/tools/charon_aeneas_test/rust"

# Create the target directory if it doesn't exist
os.makedirs(target_dir, exist_ok=True)

# Loop through each item in the input directory
for crate_name in os.listdir(input_dir):
    crate_path = os.path.join(input_dir, crate_name)
    # Ensure the item is a directory (i.e., a crate)
    if os.path.isdir(crate_path):
        # Construct the expected path for src/main.rs within the crate
        main_rs_path = os.path.join(crate_path, "src", "main.rs")
        if os.path.exists(main_rs_path):
            # The new filename is the crate name with a .rs extension
            new_filename = f"{crate_name}.rs"
            target_file = os.path.join(target_dir, new_filename)

            # Copy main.rs to the target directory with the new name
            shutil.copy(main_rs_path, target_file)
            print(f"Copied '{main_rs_path}' to '{target_file}'")
        else:
            print(f"Warning: 'src/main.rs' not found in crate '{crate_name}'.")
