# Rust for AI

> **An easy-to-set-up workspace for AI research and Jupyter notebooks in Rust.**

Write and run **Rust code in Jupyter notebooks** for simple, reproducible AI experiments. No cloud account, API key, or ML framework needed for the included examples.

## 1. Install and start

Install [Rust](https://rustup.rs/) and [Python 3.9+](https://www.python.org/downloads/) once. Then:

```sh
git clone https://github.com/edujbarrios/rust-for-ai.git
cd rust-for-ai
```

Choose your system:

| System | Start JupyterLab |
| --- | --- |
| macOS / Linux | `bash start.sh` |
| Windows (PowerShell) | `.\start.ps1` |

**That's it.** The first run sets up a local Python environment, JupyterLab, and the [Evcxr Rust kernel](https://github.com/evcxr/evcxr). Later runs simply open JupyterLab. Installing the Rust kernel the first time may take a while.

## 2. Open a notebook

Use **either** editor—your notebooks are the same `.ipynb` files:

| JupyterLab (browser) | VS Code |
| --- | --- |
| Run the start command above. JupyterLab opens in your browser. | Install the [Jupyter extension](https://marketplace.visualstudio.com/items?itemName=ms-toolsai.jupyter) in VS Code. |
| Open a notebook in `notebooks/`. | Open this repository folder in VS Code, then open a `.ipynb` file. |
| Choose the **Rust** kernel, then **Run All**. | Click **Select Kernel → Rust**, then **Run All**. |

For VS Code, **run the setup once** before selecting Rust: `bash scripts/setup.sh` on macOS/Linux or `.\scripts\setup.ps1` on Windows. If Rust does not appear, restart VS Code and select the Rust Jupyter kernel (Evcxr). No need to activate `.venv` manually.

## 3. Try the notebooks

| Notebook | Experiment |
| --- | --- |
| [01 · Hello Rust](notebooks/01_hello_rust.ipynb) | Learn Rust cells, variables and vectors |
| [02 · Random Forest](notebooks/02_random_forest.ipynb) | Generate synthetic data, train a forest, examine accuracy and a confusion matrix |
| [03 · Neural Network](notebooks/03_neural_network.ipynb) | Train a small neural network with backpropagation on XOR |

All examples use Rust's standard library. They are educational research notebooks, not production-ready AI libraries.

## Helpful commands

| Command | Use |
| --- | --- |
| `bash start.sh` / `.\start.ps1` | Install if needed and launch JupyterLab |
| `bash scripts/setup.sh` / `.\scripts\setup.ps1` | Set up the Rust kernel for VS Code without starting JupyterLab |
| `python3 scripts/validate_notebooks.py` | Check notebook structure |
| `python3 scripts/test_notebook_rust.py` | Compile and test all Rust code cells with `rustc` |

**Having trouble?** Confirm Rust (`rustc --version`) and Python (`python3 --version`, or `py -3 --version` on Windows) are installed. On some Linux distributions, you may also need `python3-venv`. Re-run setup if Rust is missing from the kernel list.

## Contribute

Want to share an AI experiment in Rust? See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[Custom BSD 3-Clause Attribution License](LICENSE) with a fourth mandatory acknowledgment condition (not the standard SPDX `BSD-3-Clause`).
