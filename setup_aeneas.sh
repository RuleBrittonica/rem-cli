#!/bin/bash
# Exit immediately if any command fails
set -e

# 1. Check for required commands
for cmd in opam git make; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Error: '$cmd' is required but not installed. Aborting."
    exit 1
  fi
done

# 2. Clone the Aeneas repository (if not already cloned)
if [ ! -d "aeneas" ]; then
  echo "Cloning Aeneas repository..."
  git clone https://github.com/AeneasVerif/aeneas.git
else
  echo "Aeneas repository already exists; skipping clone."
fi
# Change directory into the repo
cd aeneas

# 3. Set up the OCaml switch
current_switch=$(opam switch show)
if [ "$current_switch" != "4.14.2" ]; then
  echo "Creating (or switching to) opam switch 4.14.2..."
  # The following will create the switch if it does not exist,
  # or simply switch to it if it already exists.
  opam switch create 4.14.2 || opam switch 4.14.2
  # Update the environment so that the current shell uses the new switch
  eval "$(opam env)"
fi


# 4. Install OCaml dependencies
echo "Installing OCaml dependencies..."
opam install ppx_deriving visitors easy_logging zarith yojson core_unix odoc \
  ocamlgraph menhir ocamlformat unionFind -y


# 5. Build Charon
echo "Setting up Charon..."

# Modify the build scripts for Charon to remove Nix issues
sed -i.bak '/^rebuild() {/,/^}/c\
rebuild() {\
    make test\
}' scripts/check-charon-install.sh

make setup-charon

# 6. Build AENEAS and run tests
echo "Building Aeneas..."
make
make test

# 7. Copy generated binaries into the tools folder
echo "Copying binaries to the tools folder..."
cd ..
# Create the tools folder if it does not exist
mkdir -p tools
# Define the list of binaries to copy
BINARIES=("./bin/aeneas" "./charon/bin/charon" "./charon/bin/charon-driver")

for bin in "${BINARIES[@]}"; do
  # The binaries are expected to be in the aeneas/ folder.
  if [ -f "aeneas/$bin" ]; then
    cp "aeneas/$bin" tools/
    echo "Copied $bin to ./tools"
  else
    echo "Warning: Binary '$bin' not found in aeneas/."
  fi
done

echo "Setup complete! You can find the binaries in the ./tools folder."

# Cleanup by removing the aeneas/ folder
rm -rf aeneas