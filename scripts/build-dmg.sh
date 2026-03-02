#!/bin/zsh
# Build script for creating DMG file.
# The DMG output name includes the version from Cargo.toml (e.g. mac-stats_0.1.24_aarch64.dmg).
# Remember: bump version in src-tauri/Cargo.toml before building a release DMG.
# Remember: sync frontend files (src/ → src-tauri/dist/) so the DMG has latest UI; this script runs sync automatically.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Sync frontend files so the DMG bundle has the latest src/ content
echo "Syncing frontend files (src/ → src-tauri/dist/)..."
"$SCRIPT_DIR/sync-dist.sh"
echo ""

cd "$PROJECT_ROOT/src-tauri"

VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/' | tr -d ' ')
echo "Building mac-stats DMG (version $VERSION)..."
echo ""

# Ensure dist directory exists (sync-dist.sh should have populated it)
if [[ ! -d "dist" ]]; then
    echo "Error: dist directory not found. Run ./scripts/sync-dist.sh first."
    exit 1
fi

# Build the DMG
cargo tauri build --bundles dmg

# Show the result
DMG_DIR="target/release/bundle/dmg"
DMG_FILES=($DMG_DIR/mac-stats_*.dmg(N))

if [[ ${#DMG_FILES[@]} -gt 0 ]]; then
    DMG_FILE="${DMG_FILES[1]}"
    echo ""
    echo "✅ DMG created successfully! (filename includes version from Cargo.toml)"
    echo "Location: $DMG_FILE"
    ls -lh "$DMG_FILE"
else
    echo "❌ DMG not found. Build may have failed."
    exit 1
fi
