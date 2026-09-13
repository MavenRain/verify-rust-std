"""Check that the proof's original method bodies match this std checkout."""

from pathlib import Path
import sys


def method(source: str, signature: str) -> str:
    start_marker = "    " + signature + " {\n"
    if source.count(start_marker) != 1:
        sys.exit(f"Expected exactly one method with signature: {signature}")
    start = source.index(start_marker)
    end = source.index("\n    }", start) + len("\n    }")
    return source[start:end]


proof_dir = Path(__file__).resolve().parent
repo_dir = proof_dir.parents[1]
original = (proof_dir / "implementations/original/lib.rs").read_text()
for relative_path, signature in (
    (
        "library/core/src/slice/mod.rs",
        "pub const fn first_chunk<const N: usize>(&self) -> Option<&[T; N]>",
    ),
    (
        "library/core/src/ptr/const_ptr.rs",
        "pub const fn cast_array<const N: usize>(self) -> *const [T; N]",
    ),
):
    source = (repo_dir / relative_path).read_text()
    if method(source, signature) != method(original, signature):
        sys.exit(f"The proof snapshot differs from {relative_path}: {signature}")
    print(f"PASS: source binding for {signature}")
