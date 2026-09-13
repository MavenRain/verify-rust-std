# Generic slice backend

This is an experimental extension of VeriFast 26.01 for challenge 0017 and
PR #603. It is not yet evidence that the challenge has been completed.

The backend patch connects shared and exclusive slice references to element
ownership and lifetime borrows. It also translates symbolic array lengths and
defines ownership of `[T; N]`. Its regressions use abstract `T`, symbolic slice
lengths, and symbolic `N`. They do not allocate bounded backing arrays or
enumerate element types. Shared elements use `T.share`, including for interior
mutable types.

Slice data pointers explicitly discard length metadata. The validity predicate
records Rust's nonnegative layout sizes, including zero-sized types, and the
shared-reference ownership bridge accepts unsized pointees. A rejection case
checks that bounds alone cannot manufacture shared element ownership.

Rust's const-argument type predicates are retained in the exported MIR. The
translator uses `const N: usize` to establish the corresponding integer bounds;
other const-parameter types remain unsupported. The refinement checker compares
both the constant and its type, and a rejection fixture changes `usize` to `u8`.
The tuple ownership bridge follows the existing four-field tuple bridge: both
fields must be owned before the pair can be packaged. This library specification
is part of the trusted model and has a missing-field-ownership rejection test.

The sharing regression splits a symbolic-length slice by induction over its
element-sharing predicate. A second inductive lemma preserves validity when
taking a suffix, including its alignment and pointer limits. It uses the
existing Rust layout model and applies to zero-sized element types.
Owned splitting and joining lemmas preserve every element's ownership, and
conversion lemmas reuse the existing memory rules for arrays. The trusted value
model adds a round-trip rule for `Array_of_elems` guarded by exact length equality.
A rejection test attempts conversion from too few elements.
Array conversion also requires a non-null pointer: an empty element region
alone cannot justify the array storage predicate, which implies non-nullness.
A rejection test guards against deriving a contradiction from empty storage
at null, and a positive test round-trips an empty array at a non-null pointer.
The ghost parameter representing const `N` carries no `Sized` requirement;
only the element type needs that bound.
Owned suffix validity uses induction over the element list. It preserves the
whole slice's ownership and reuses the shared proof's pointer arithmetic lemma.
The array-alignment rule follows the [Rust Reference's array layout guarantee](https://doc.rust-lang.org/reference/type-layout.html#array-layout),
including empty arrays. This rule is also part of the trusted model.
Rejection cases attempt to manufacture a borrow
without memory or without ownership of its initialized elements. They must
reach a missing-ownership error; a timeout or an unsupported feature fails CI.

The compiler build runs only on a GitHub-hosted Linux runner. The build script
refuses local execution. Cargo and Make use two workers, and the workflow has a
timeout.
The challenge commands do not enable assumption, unspecified-function skipping,
reference-creation bypass, or unwind-path bypass options. CI also runs the upstream Rust and
refinement regression suites with their existing test options and two workers.
All backend tests and challenge proofs enable `-check_raw_mut_ref_creation`.
The pinned frontend otherwise skips mutable-reference creation automatically.
The new mode routes creation from non-reference places through `create_ref_mut`,
which requires full initialized storage, non-nullness and alignment, and returns
a restoration token while preserving the address. A null-pointer regression
uses an empty array without assuming storage at null. The misaligned-array
regression supplies storage and must still fail the alignment check.
It rejects raw mutable references to unsized pointees until those checks are
implemented. Regressions require rejection without storage or with only a
fraction of the required ownership.

The verified implementations are `first_chunk` and `first_chunk_mut`, together
with their `cast_array` helpers. At commit `c0129c88`, [remote CI](https://github.com/MavenRain/verify-rust-std/actions/runs/34749040627)
passed verification, exact source binding, MIR refinement, all backend tests,
and both pinned upstream regression suites. The proof uses generic `T` and
symbolic slice lengths and `N`. The remaining challenge functions still need
source-bound proofs.
CI compares all four original method bodies with the standard-library checkout
and runs the existing MIR refinement checker between the original and
annotated copies.
Both copies include the same proof-only module; its contents are ghost code.
The annotated `cast_array` contract requires pointer preservation and no
unwinding; CI checks its body against that contract. `first_chunk` packages
result ownership after the Rust expression has evaluated. Reference creation
uses the existing `precreate_ref` and `init_ref_share` rules, retaining the
required lifetime tokens. A rejection test omits the element-sharing predicate.

The `first_chunk_mut` proof splits owned elements, creates a checked mutable array
reference, and proves restoration of the whole slice after the borrow ends.
Its source binding, verification and MIR refinement passed in the same remote run.

`backend.patch` applies to the pinned source. `apply-array-value-fix.py` applies
the array-local change and reference-check option wiring with exact byte
preimages because those upstream lines use CRLF endings. The repository's
whitespace checks remain enabled.

Required follow-up before claiming challenge coverage:

1. Review the new trusted ownership rules and the backend's safety coverage.
2. Add proofs of the remaining challenge functions and bind their bodies to the exact
    standard-library source using the existing refinement-checker workflow.
3. Run each added proof and the existing VeriFast regression suites remotely.

The tests in this directory exercise the backend. They are not substitutes for
proofs of the standard-library implementations.

VeriFast 26.01 also documents incomplete checks for Rust's mutable-reference
creation, aliasing, and function-call reference protection in
[`tests/rust/README.md`](https://github.com/verifast/verifast/blob/dcfad5bd4c147117bc1dfd9ede0298c6de60fed0/tests/rust/README.md).
The new slice ownership predicates do not establish conformance to all of
Rust's aliasing rules. The raw mutable-reference check addresses one part of
reference creation; reborrow and call-protection limitations remain.
These limitations must be addressed when assessing the
challenge's undefined-behavior coverage, especially for mutable operations.
