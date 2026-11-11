"""Compile and run the Rust code cells in all checked-in notebooks.

This complements notebook JSON validation. It does not replace a live Evcxr
kernel integration test, but catches Rust syntax/runtime regressions offline.
"""
import json
import os
from pathlib import Path
import subprocess
import tempfile


def main() -> None:
    notebooks = sorted(Path("notebooks").glob("*.ipynb"))
    if not notebooks:
        raise SystemExit("No notebooks found")

    for path in notebooks:
        notebook = json.loads(path.read_text(encoding="utf-8"))
        code = "\n\n".join(
            "".join(cell["source"])
            for cell in notebook["cells"]
            if cell["cell_type"] == "code"
        )
        # These tutorials intentionally use std-only Rust (no Evcxr magics).
        with tempfile.TemporaryDirectory() as temporary:
            temp = Path(temporary)
            source = temp / "notebook.rs"
            binary = temp / ("notebook.exe" if __import__("os").name == "nt" else "notebook")
            source.write_text("fn main() {\n" + code + "\n}\n", encoding="utf-8")
            subprocess.run(
                ["rustc", "--edition=2021", "-o", str(binary), str(source)],
                check=True, timeout=120,
            )
            result = subprocess.run(
                [str(binary)], capture_output=True, text=True, check=True, timeout=120,
                env={**os.environ, "RUST_FOR_AI_CNN_OFFLINE": "1"},
            )
            print(f"PASS: {path}")
            print(result.stdout.strip())


if __name__ == "__main__":
    main()
