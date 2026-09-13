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
element-sharing predicate. Rejection cases attempt to manufacture a borrow
without memory or without ownership of its initialized elements. They must
reach a missing-ownership error; a timeout or an unsupported feature fails CI.

The compiler build runs only on a GitHub-hosted Linux runner. The build script
refuses local execution. Cargo and Make use two workers, and the workflow has a
timeout.
Verification does not enable assumptions, skip unspecified functions, ignore
reference creation, or ignore unwind paths.

The first implementation under development is `first_chunk`, together with its
`cast_array` helper. CI compares both original method bodies with the standard
library checkout and runs the existing MIR refinement checker between the
original and annotated copies. This implementation is not counted as proved
until its verification passes.

`backend.patch` applies to the pinned source. `apply-array-value-fix.py` applies
the array-local change with an exact byte preimage because those upstream lines
use CRLF endings. The repository's whitespace checks remain enabled.

Required follow-up before claiming challenge coverage:

1. Check the new ownership rules and their rejection regressions.
2. Check the symbolic const-parameter and array-ownership regressions.
3. Add proofs of the challenge functions and bind their bodies to the exact
   standard-library source using the existing refinement-checker workflow.
4. Run the proofs and the existing VeriFast regression suite remotely.

The tests in this directory exercise the backend. They are not substitutes for
proofs of the standard-library implementations.
