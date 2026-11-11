# Executed Rust notebooks

These are **results produced by GitHub Actions using the actual Evcxr Rust
Jupyter kernel**. They are not manually fabricated output.

- [01 — Hello Rust](01_hello_rust.ipynb)
- [02 — Random Forest](02_random_forest.ipynb)
- [03 — Neural Network (XOR)](03_neural_network.ipynb)

The original notebooks live in [`notebooks/`](../notebooks/) and remain clean
so new users can run them from scratch using `bash start.sh` or
`.\start.ps1`.

To recreate these output-bearing notebooks locally, first run the setup script,
then install `nbclient` and `nbformat` into `.venv` and run:

```sh
.venv/bin/python -m pip install nbclient nbformat
.venv/bin/python scripts/execute_notebooks.py
```

On GitHub, the [Execute Rust notebooks workflow](../.github/workflows/execute-notebooks.yml)
runs the same process and commits the executed copies to the demo branch
if all three notebooks complete successfully.
