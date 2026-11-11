"""Execute the tutorial notebooks using the real Evcxr Jupyter Rust kernel.

Writes separate, GitHub-renderable copies under executed/; source notebooks
remain clean and continue to pass scripts/validate_notebooks.py.
"""
from pathlib import Path

import nbformat
from nbclient import NotebookClient

ROOT = Path(__file__).resolve().parent.parent
SOURCE = ROOT / "notebooks"
DEST = ROOT / "executed"


def main() -> None:
    DEST.mkdir(exist_ok=True)
    paths = sorted(SOURCE.glob("*.ipynb"))
    if not paths:
        raise SystemExit("No source notebooks found")

    for path in paths:
        print(f"Executing {path.name} using the Rust Jupyter kernel", flush=True)
        notebook = nbformat.read(path, as_version=4)
        client = NotebookClient(
            notebook,
            kernel_name="rust",
            timeout=900,
            startup_timeout=180,
            resources={"metadata": {"path": str(SOURCE)}},
            allow_errors=False,
        )
        result = client.execute()
        code = [cell for cell in result.cells if cell.cell_type == "code"]
        assert code, f"{path}: no code cells"
        assert all(cell.execution_count is not None for cell in code), (
            f"{path}: a code cell was not executed"
        )
        assert all(
            not any(output.output_type == "error" for output in cell.outputs)
            for cell in code
        ), f"{path}: a code cell failed"
        target = DEST / path.name
        nbformat.write(result, target)
        print(f"Saved executed notebook: {target.relative_to(ROOT)}", flush=True)

    print(f"Executed {len(paths)} Rust notebooks successfully.", flush=True)


if __name__ == "__main__":
    main()
