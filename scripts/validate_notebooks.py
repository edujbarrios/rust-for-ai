"""Validate checked-in Jupyter notebooks without third-party Python dependencies."""

import json
from pathlib import Path


def main() -> None:
    notebooks = sorted(Path("notebooks").glob("*.ipynb"))
    if not notebooks:
        raise SystemExit("No notebooks found in notebooks/")

    for path in notebooks:
        notebook = json.loads(path.read_text(encoding="utf-8"))
        assert notebook.get("nbformat") == 4, f"{path}: unsupported notebook version"
        kernel = notebook.get("metadata", {}).get("kernelspec", {})
        assert kernel.get("name") == "rust", f"{path}: expected Rust kernel"
        cells = notebook.get("cells", [])
        assert cells, f"{path}: empty notebook"
        assert any(cell.get("cell_type") == "code" for cell in cells), f"{path}: no code cells"
        for cell in cells:
            assert cell.get("cell_type") in ("markdown", "code"), f"{path}: invalid cell type"
            assert isinstance(cell.get("source"), list), f"{path}: source must be a list"
            if cell["cell_type"] == "code":
                assert cell.get("execution_count") is None, f"{path}: commit notebooks unexecuted"
                assert cell.get("outputs") == [], f"{path}: strip outputs before committing"
        print(f"OK: {path} ({len(cells)} cells)")

    print(f"Validated {len(notebooks)} Rust notebooks.")


if __name__ == "__main__":
    main()
