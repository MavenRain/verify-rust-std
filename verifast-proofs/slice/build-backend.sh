#!/usr/bin/env bash
set -euo pipefail

if [[ "${GITHUB_ACTIONS:-}" != true || "$(uname -s)" != Linux ]]; then
    echo 'Build this experimental backend on a GitHub-hosted Linux runner.' >&2
    exit 2
fi

backend_dir=$(cd -- "${1:?expected VeriFast checkout}" && pwd)
proof_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
python3 -I "$proof_dir/check-source.py"
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
python3 -I "$proof_dir/apply-array-value-fix.py" "$backend_dir"
cd -- "$backend_dir"
./setup-build.sh
source ./config.sh
export PATH="/tmp/$VFDEPS_NAME/bin:$PATH"
export LD_LIBRARY_PATH="/tmp/$VFDEPS_NAME/lib:${LD_LIBRARY_PATH:-}"
make -C src -j2 verifast
make -C src -j2 ../bin/vf-rust-mir-exporter
make -C src -j2 ../bin/refinement-checker

mkdir -p -- "$proof_dir/results"
verification_failed=0
for test_file in "$proof_dir"/tests/*.rs; do
    test_name=$(basename -- "$test_file" .rs)
    test_log="$proof_dir/results/$test_name.log"
    if timeout 60 bin/verifast -prover Redux -check_raw_mut_ref_creation \
        -rustc_arg --crate-type=lib -rustc_arg -Zthreads=1 \
        "$test_file" >"$test_log" 2>&1; then
        verification_status=0
    else
        verification_status=$?
    fi
    cat "$test_log"
    case "$test_name:$verification_status" in
        reject_null_array_ref:1|reject_misaligned_array_ref:1)
            test_output=$(<"$test_log")
            if [[ "$test_output" == *'aliasing.rsspec('*'error: Cannot prove condition.'* ]]; then
                echo "PASS: $test_name rejected for invalid reference geometry"
            else
                echo "FAIL: $test_name did not reach the expected reference geometry check"
                verification_failed=1
            fi
            ;;
        unsupported_raw_mut_slice:1)
            test_output=$(<"$test_log")
            if [[ "$test_output" == *'error: Checked raw mutable references to unsized pointees are not yet supported'* ]]; then
                echo 'PASS: unsupported raw mutable slice reference rejected'
            else
                echo 'FAIL: raw mutable slice did not reach the expected unsupported check'
                verification_failed=1
            fi
            ;;
        unsupported_raw_mut_slice:*)
            echo "FAIL: raw mutable slice returned $verification_status (expected explicit rejection)"
            verification_failed=1
            ;;
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
implementation_log="$proof_dir/results/first_chunks.log"
if timeout 60 bin/verifast -prover Redux -check_raw_mut_ref_creation \
    -rustc_arg --crate-type=lib -rustc_arg -Zthreads=1 \
    "$proof_dir/implementations/verified/lib.rs" >"$implementation_log" 2>&1; then
    echo 'PASS: first_chunk and first_chunk_mut implementations'
else
    echo 'FAIL: first_chunk and first_chunk_mut implementations'
    verification_failed=1
fi
cat "$implementation_log"

refinement_log="$proof_dir/results/refinement.log"
if timeout 60 bin/refinement-checker --rustc-args '--crate-type=lib -Zthreads=1' \
    "$proof_dir/implementations/original/lib.rs" \
    "$proof_dir/implementations/verified/lib.rs" >"$refinement_log" 2>&1; then
    echo 'PASS: implementation refinement'
else
    echo 'FAIL: implementation refinement'
    verification_failed=1
fi
cat "$refinement_log"

for fixture in verified wrong_type wrong_value; do
    fixture_log="$proof_dir/results/refinement_$fixture.log"
    if timeout 60 bin/refinement-checker --rustc-args '--crate-type=lib -Zthreads=1' \
        "$proof_dir/tests/refinement/original.rs" \
        "$proof_dir/tests/refinement/$fixture.rs" >"$fixture_log" 2>&1; then
        fixture_status=0
    else
        fixture_status=$?
    fi
    cat "$fixture_log"
    case "$fixture:$fixture_status" in
        verified:0) echo 'PASS: const-generic refinement' ;;
        wrong_type:1|wrong_type:2)
            fixture_output=$(<"$fixture_log")
            if [[ "$fixture_output" == *'ERROR: The two functions have different const-argument type predicates'* ]]; then
                echo 'PASS: changed const-argument type rejected'
            else
                echo 'FAIL: changed const type did not reach the expected refinement check'
                verification_failed=1
            fi
            ;;
        wrong_value:1)
            fixture_output=$(<"$fixture_log")
            if [[ "$fixture_output" == *'the return values'*'are not equal'* ]]; then
                echo 'PASS: changed const-argument value rejected'
            else
                echo 'FAIL: changed const value did not reach the expected refinement check'
                verification_failed=1
            fi
            ;;
        *)
            echo "FAIL: refinement fixture $fixture returned $fixture_status"
            verification_failed=1
            ;;
    esac
done

upstream_log="$proof_dir/results/upstream_refinement.log"
if make -C src -j2 ../bin/mysh >"$upstream_log" 2>&1 && (
    cd tests/rust/refinement_checker
    export PATH="$backend_dir/bin:$PATH"
    timeout 300 "$backend_dir/bin/mysh" -cpus 2 < testsuite.mysh
) >>"$upstream_log" 2>&1; then
    echo 'PASS: upstream refinement suite'
else
    echo 'FAIL: upstream refinement suite'
    verification_failed=1
fi
cat "$upstream_log"

upstream_rust_log="$proof_dir/results/upstream_rust.log"
if make -C src -j2 ../bin/rustc-verifast >"$upstream_rust_log" 2>&1 &&
    make -C src -j2 ../bin/cargo-verifast >>"$upstream_rust_log" 2>&1 && (
    cd tests/rust
    export PATH="$backend_dir/bin:$PATH"
    timeout 900 "$backend_dir/bin/mysh" -cpus 2 < testsuite.mysh
) >>"$upstream_rust_log" 2>&1; then
    echo 'PASS: upstream Rust suite'
else
    echo 'FAIL: upstream Rust suite'
    verification_failed=1
fi
cat "$upstream_rust_log"
exit "$verification_failed"
