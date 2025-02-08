import os

# Define the replacements for specific keys
replacements = {
    "edition.workspace = true": 'edition = "2021"',
    "rust-version.workspace = true": 'rust-version = "1.80"',
    "authors.workspace = true": 'authors = ["N/A"]',
    "license.workspace = true": 'license = "MIT"',
    "repository.workspace = true": 'repository = "https://github.com/RuleBrittonica/rem-testfiles"',
    "description.workspace = true": 'description = "Test Data"'
}

def replace_and_remove_lints_in_file(file_path):
    with open(file_path, "r") as f:
        lines = f.readlines()

    updated_lines = []
    skip_lints = False
    changed = False

    for line in lines:
        stripped_line = line.strip()

        # Detect and remove the `[lints]` section
        if stripped_line == "[lints]":
            skip_lints = True
            changed = True
            continue
        if skip_lints:
            if stripped_line == "workspace = true":
                continue 
            else:
                skip_lints = False

        for key, value in replacements.items():
            if stripped_line == key:
                line = line.replace(key, value + "\n")
                changed = True

        updated_lines.append(line)

    if changed:
        with open(file_path, "w") as f:
            f.writelines(updated_lines)
        print(f"Updated: {file_path}")
    else:
        print(f"No changes needed: {file_path}")

def process_directory(directory):
    for root, _, files in os.walk(directory):
        for file in files:
            if file == "Cargo.toml":
                file_path = os.path.join(root, file)
                print(f"Processing: {file_path}")
                replace_and_remove_lints_in_file(file_path)

if __name__ == "__main__":
    target_directory = "./src_tests/extract"
    process_directory(target_directory)
