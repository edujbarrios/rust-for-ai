"""Run the tutorials in the real Evcxr Rust Jupyter kernel and save outputs.

Update both the original notebooks (the README's primary links) and an
executed/ gallery so GitHub displays real cell results in either location.
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
        assert any(cell.outputs for cell in code), f"{path}: no visible output"

        # Save to the ORIGINAL notebook, not only to a separate copy.
        nbformat.write(result, path)
        target = DEST / path.name
        nbformat.write(result, target)
        print(f"Saved executed notebook: {path.relative_to(ROOT)}", flush=True)

    print(f"Executed {len(paths)} Rust notebooks successfully.", flush=True)


if __name__ == "__main__":
    main()
