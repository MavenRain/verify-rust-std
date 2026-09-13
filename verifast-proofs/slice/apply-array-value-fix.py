"""Apply the Rust array-local fix to the pinned VeriFast source."""

from pathlib import Path
import sys


source_path = Path(sys.argv[1]) / "src/verifast.ml"
source = source_path.read_bytes()
# Escapes preserve the upstream preimage's CRLF endings without a lint waiver.
before = (
    b"            StaticArrayType (elemTp, elemCount) ->\r\n"
    b"            produce_object t\r\n"
)
after = (
    b"            StaticArrayType (elemTp, elemCount) when not is_rust || !address_taken ->\n"
    b"            produce_object t\n"
)
if source.count(before) != 1:
    sys.exit("The pinned Rust array-local preimage did not match exactly once.")
source_path.write_bytes(source.replace(before, after))
