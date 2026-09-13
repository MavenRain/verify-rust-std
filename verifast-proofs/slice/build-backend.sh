#!/usr/bin/env bash
set -euo pipefail

if [[ "${GITHUB_ACTIONS:-}" != true || "$(uname -s)" != Linux ]]; then
    echo 'Build this experimental backend on a GitHub-hosted Linux runner.' >&2
    exit 2
fi

backend_dir=$(cd -- "${1:?expected VeriFast checkout}" && pwd)
proof_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
export CARGO_BUILD_JOBS=2
export MAKEFLAGS=-j2

git -C "$backend_dir" apply --check "$proof_dir/backend.patch"
git -C "$backend_dir" apply "$proof_dir/backend.patch"
cd -- "$backend_dir"
./setup-build.sh
source ./config.sh
export PATH="/tmp/$VFDEPS_NAME/bin:$PATH"
export LD_LIBRARY_PATH="/tmp/$VFDEPS_NAME/lib:${LD_LIBRARY_PATH:-}"
make -C src -j2 verifast
make -C src -j2 ../bin/vf-rust-mir-exporter

mkdir -p -- "$proof_dir/results"
for test_file in "$proof_dir"/tests/*.rs; do
    test_name=$(basename -- "$test_file" .rs)
    timeout 60 bin/verifast -prover Redux \
        -rustc_arg --crate-type=lib -rustc_arg -Zthreads=1 \
        "$test_file" 2>&1 | tee "$proof_dir/results/$test_name.log"
done
