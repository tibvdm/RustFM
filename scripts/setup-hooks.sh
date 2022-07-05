#!/bin/sh

# Navigate to 'scripts' folder
pushd $(dirname "$0")

# Move hooks to the 'git hooks' folder
cp ../.hooks/* ../.git/hooks/

# Return to original location
popd
