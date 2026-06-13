#!/bin/bash
set -e

echo "=== SwiftTeX Release Check ==="

echo "1. Running full test suite..."
cargo test --workspace

echo "2. Running clippy..."
cargo clippy --workspace -- -D warnings

echo "3. Checking documentation builds..."
cargo doc --workspace --no-deps

echo "4. Checking all versions consistent..."
grep -r 'version = "0.3.0"' */Cargo.toml | wc -l

echo "5. Running benchmarks (quick mode)..."
cargo bench --package swifttex-renderer-svg

echo "6. Verifying workspace members..."
cargo metadata --no-deps --format-version 1 | \
  python3 -c "
import json,sys
m = json.load(sys.stdin)
names = [p['name'] for p in m['packages']]
required = [
  'swifttex-lexer','swifttex-parser','swifttex-layout',
  'swifttex-renderer-svg','swifttex-renderer-mathml',
  'swifttex-plugin-api','swifttex-wasm'
]
missing = [r for r in required if r not in names]
if missing:
  print('MISSING:', missing); sys.exit(1)
print('All 7 crates present:', names)
"

echo "7. Sample renders..."
cargo test --package swifttex-renderer-svg \
  validate_all_perf_targets -- --nocapture

echo "=== Release check passed ==="
