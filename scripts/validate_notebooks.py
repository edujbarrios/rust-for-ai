"""Validate notebook structure and checked-in Rust execution outputs."""
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
        code = [cell for cell in cells if cell["cell_type"] == "code"]
        assert code, f"{path}: no code cells"
        for cell in cells:
            assert cell.get("cell_type") in ("markdown", "code"), f"{path}: invalid cell type"
            assert isinstance(cell.get("source"), list), f"{path}: source must be a list"
        # Clean source notebooks are valid before first execution, but any
        # committed execution must be complete, with no error outputs.
        counts = [cell.get("execution_count") for cell in code]
        assert all(x is None for x in counts) or all(
            isinstance(x, int) and x > 0 for x in counts
        ), f"{path}: partially executed notebook"
        assert all(
            not any(out.get("output_type") == "error" for out in cell.get("outputs", []))
            for cell in code
        ), f"{path}: error output detected"
        if all(x is not None for x in counts):
            assert any(cell.get("outputs") for cell in code), f"{path}: missing visible outputs"
        print(f"OK: {path} ({len(cells)} cells, executed={counts[0] is not None})")

    print(f"Validated {len(notebooks)} Rust notebooks.")


if __name__ == "__main__":
    main()
