# Generic slice backend

This is an experimental extension of VeriFast 26.01 for challenge 0017 and
PR #603. It is not yet evidence that the challenge has been completed.

The backend patch connects shared and exclusive slice references to element
ownership and lifetime borrows. It also translates symbolic array lengths and
defines ownership of `[T; N]`. Its regressions use abstract `T`, symbolic slice
lengths, and symbolic `N`. They do not allocate bounded backing arrays or
enumerate element types. Shared elements use `T.share`, including for interior
mutable types.

The compiler build runs only on a GitHub-hosted Linux runner. The build script
refuses local execution. It uses two build workers and a workflow timeout.
Verification does not enable assumptions, skip unspecified functions, ignore
reference creation, or ignore unwind paths.

Required follow-up before claiming challenge coverage:

1. Check the new ownership rules and their rejection regressions.
2. Check the symbolic const-parameter and array-ownership regressions.
3. Add proofs of the challenge functions and bind their bodies to the exact
   standard-library source using the existing refinement-checker workflow.
4. Run the proofs and the existing VeriFast regression suite remotely.

The tests in this directory exercise the backend. They are not substitutes for
proofs of the standard-library implementations.
