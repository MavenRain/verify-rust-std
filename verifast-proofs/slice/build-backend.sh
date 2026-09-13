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

expected_revision=dcfad5bd4c147117bc1dfd9ede0298c6de60fed0
if [[ "$(git -C "$backend_dir" rev-parse HEAD)" != "$expected_revision" ]]; then
    echo 'The backend patch requires the pinned VeriFast 26.01 revision.' >&2
    exit 2
fi
git -C "$backend_dir" diff HEAD --exit-code
git -C "$backend_dir" apply --unidiff-zero --check "$proof_dir/backend.patch"
git -C "$backend_dir" apply --unidiff-zero "$proof_dir/backend.patch"
cd -- "$backend_dir"
./setup-build.sh
source ./config.sh
export PATH="/tmp/$VFDEPS_NAME/bin:$PATH"
export LD_LIBRARY_PATH="/tmp/$VFDEPS_NAME/lib:${LD_LIBRARY_PATH:-}"
make -C src -j2 verifast
make -C src -j2 ../bin/vf-rust-mir-exporter

mkdir -p -- "$proof_dir/results"
verification_failed=0
for test_file in "$proof_dir"/tests/*.rs; do
    test_name=$(basename -- "$test_file" .rs)
    test_log="$proof_dir/results/$test_name.log"
    if timeout 60 bin/verifast -prover Redux \
        -rustc_arg --crate-type=lib -rustc_arg -Zthreads=1 \
        "$test_file" >"$test_log" 2>&1; then
        verification_status=0
    else
        verification_status=$?
    fi
    cat "$test_log"
    case "$test_name:$verification_status" in
        reject_*:1)
            test_output=$(<"$test_log")
            if [[ "$test_output" == *'error: No matching heap chunks:'* ]]; then
                echo "PASS: $test_name rejected for missing ownership"
            else
                echo "FAIL: $test_name did not reach the expected ownership check"
                verification_failed=1
            fi
            ;;
        reject_*:*)
            echo "FAIL: $test_name returned $verification_status (expected ownership rejection)"
            verification_failed=1
            ;;
        *:0) echo "PASS: $test_name" ;;
        *)
            echo "FAIL: $test_name returned $verification_status"
            verification_failed=1
            ;;
    esac
done
exit "$verification_failed"
