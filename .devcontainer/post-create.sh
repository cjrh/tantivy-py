#!/usr/bin/env bash
set -euo pipefail

# Install dev dependencies
pip install -r requirements-dev.txt

# Build the native extension in development mode
maturin develop

echo "Dev container ready. Run 'pytest' to verify."
