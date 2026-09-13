"""Apply exact byte patches to pinned VeriFast sources with CRLF endings."""

from pathlib import Path
import sys


backend = Path(sys.argv[1])

# Escapes preserve the upstream preimages' CRLF endings without a lint waiver.
patches = (
    (
        "src/verifast.ml",
        (
            (
                b"            StaticArrayType (elemTp, elemCount) ->\r\n"
                b"            produce_object t\r\n",
                b"            StaticArrayType (elemTp, elemCount) when not is_rust || !address_taken ->\n"
                b"            produce_object t\n",
            ),
            (
                b"            let ignore_ref_creation = Vfbindings.get Vfparam_ignore_ref_creation vfbindings\r\n",
                b"            let ignore_ref_creation = Vfbindings.get Vfparam_ignore_ref_creation vfbindings\n"
                b"            let check_raw_mut_ref_creation = Vfbindings.get Vfparam_check_raw_mut_ref_creation vfbindings\n",
            ),
        ),
    ),
    (
        "src/frontend/verifast0.ml",
        (
            (
                b"| Vfparam_ignore_ref_creation: bool vfparam\r\n",
                b"| Vfparam_ignore_ref_creation: bool vfparam\n"
                b"| Vfparam_check_raw_mut_ref_creation: bool vfparam\n",
            ),
            (
                b"  | Vfparam_ignore_ref_creation, Vfparam_ignore_ref_creation -> Some a0\r\n",
                b"  | Vfparam_ignore_ref_creation, Vfparam_ignore_ref_creation -> Some a0\n"
                b"  | Vfparam_check_raw_mut_ref_creation, Vfparam_check_raw_mut_ref_creation -> Some a0\n",
            ),
            (
                b"| Vfparam_ignore_ref_creation -> BoolParam\r\n",
                b"| Vfparam_ignore_ref_creation -> BoolParam\n"
                b"| Vfparam_check_raw_mut_ref_creation -> BoolParam\n",
            ),
            (
                b'  "ignore_ref_creation", (Vfparam Vfparam_ignore_ref_creation, "In Rust, treat &E or &mut E like &raw E. This is unsound!");\r\n',
                b'  "ignore_ref_creation", (Vfparam Vfparam_ignore_ref_creation, "In Rust, treat &E or &mut E like &raw E. This is unsound!");\n'
                b'  "check_raw_mut_ref_creation", (Vfparam Vfparam_check_raw_mut_ref_creation, "In Rust, check mutable-reference creation from non-reference places, including raw pointer dereferences. Unsized pointees are not supported in this mode.");\n',
            ),
        ),
    ),
)

updates = []
for relative_path, replacements in patches:
    source_path = backend / relative_path
    source = source_path.read_bytes()
    for before, after in replacements:
        if source.count(before) != 1:
            sys.exit(f"The pinned {relative_path} preimage did not match exactly once.")
        source = source.replace(before, after)
    updates.append((source_path, source))

for source_path, source in updates:
    source_path.write_bytes(source)
