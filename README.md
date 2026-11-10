# Rust for AI

> **"my easy-setup to work with AI-research and AI notebook projects on rust "**

A **notebook-first** Rust workspace for hands-on AI research: write Rust in JupyterLab, experiment on synthetic data, and share reproducible notebooks. No proprietary API keys or cloud services required.

## Get started

Install [Rust](https://rustup.rs/) and [Python 3.9+](https://www.python.org/downloads/) once, then clone the repo:

```sh
git clone https://github.com/edujbarrios/rust-for-ai.git
cd rust-for-ai
```

**Start JupyterLab with one command:**

| Platform | Command |
| --- | --- |
| macOS / Linux | `bash start.sh` |
| Windows (PowerShell) | `.\start.ps1` |

The **first run** automatically creates a local `.venv`, installs JupyterLab, installs/registers the [Evcxr](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md) Rust kernel, and opens JupyterLab. Later runs just launch Jupyter. The first kernel build may take several minutes. No need to activate a virtual environment manually.

> Inspect [`scripts/setup.sh`](scripts/setup.sh) or [`scripts/setup.ps1`](scripts/setup.ps1) if you want to review what gets installed. Rust and Python are prerequisites; the setup does not install them globally.

Select the **Rust** kernel in Jupyter, then open any notebook:

## Learn with real experiments

| Notebook | What you'll do |
| --- | --- |
| [01 · Hello Rust](notebooks/01_hello_rust.ipynb) | Variables, vectors and arithmetic in Rust cells |
| [02 · Random Forest](notebooks/02_random_forest.ipynb) | Generate synthetic nonlinear data, train 21 randomized decision trees, test accuracy and confusion matrix |
| [03 · Neural Network](notebooks/03_neural_network.ipynb) | Learn XOR using a neural network, sigmoid/tanh, gradient descent and backpropagation |

All examples run locally with Rust's standard library. The algorithms are intentionally educational, not production ML packages.

## Useful commands

| Command | Purpose |
| --- | --- |
| `bash start.sh` / `.\start.ps1` | Set up on first run and launch JupyterLab |
| `bash scripts/setup.sh` / `.\scripts\setup.ps1` | Re-run setup if needed |
| `.venv/bin/jupyter kernelspec list` | Inspect kernels on macOS/Linux |
| `.\.venv\Scripts\jupyter.exe kernelspec list` | Inspect kernels on Windows |
| `python3 scripts/validate_notebooks.py` | Validate notebooks' metadata |
| `python3 scripts/test_notebook_rust.py` | Compile and run Rust notebook cells (requires `rustc`) |

**Troubleshooting:** If Rust or Python is not found, install the prerequisite and restart the terminal. On Linux, Python may require your distribution's `python3-venv` package. If Rust does not appear as a kernel, rerun the setup script. On Windows, if script execution is restricted, review your PowerShell execution policy rather than disabling security for the whole system.

## Contributing

Share **small, reproducible Rust notebooks**, setup fixes or AI research experiments. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[Custom BSD 3-Clause Attribution License](LICENSE), including a fourth mandatory acknowledgment condition (not the standard SPDX `BSD-3-Clause`).
