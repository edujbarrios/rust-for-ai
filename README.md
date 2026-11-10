# Rust for AI

> **"my easy-setup to work with AI-research and AI notebook projects on rust "**

**Start researching and experimenting with AI in Rust, directly in Jupyter notebooks.** No custom framework, no unnecessary setup: Rust + JupyterLab + the [Evcxr Rust kernel](https://github.com/evcxr/evcxr).

## Quick start

You need [Rust](https://rustup.rs/) and [Python 3](https://www.python.org/downloads/) installed.

**macOS / Linux**

```sh
git clone https://github.com/edujbarrios/rust-for-ai.git
cd rust-for-ai
bash scripts/setup.sh
```

**Windows (PowerShell)**

```powershell
git clone https://github.com/edujbarrios/rust-for-ai.git
cd rust-for-ai
.\scripts\setup.ps1
```

Then open JupyterLab:

| System | Command |
| --- | --- |
| macOS / Linux | `./.venv/bin/jupyter lab` |
| Windows | `.\.venv\Scripts\jupyter.exe lab` |

Select the **Rust** kernel and open [`notebooks/01_hello_rust.ipynb`](notebooks/01_hello_rust.ipynb).

> The first setup compiles the Rust kernel and can take time. The setup scripts install JupyterLab inside `.venv`, install `rust-src`, install `evcxr_jupyter`, and register its Rust kernel. You can review the scripts before running them.

## Notebooks: from zero to AI experiments

| Notebook | What you'll do |
| --- | --- |
| [01 — Hello Rust](notebooks/01_hello_rust.ipynb) | Run your first Rust cells, variables and collections |
| [02 — Linear regression](notebooks/02_linear_regression.ipynb) | Fit a line with ordinary least squares and predict a result |

These examples run locally and use only Rust's standard library. No API keys, GPUs or model downloads are needed.

## Useful commands

| Command | Purpose |
| --- | --- |
| `jupyter lab` (inside `.venv`) | Start the notebook interface |
| `jupyter kernelspec list` (inside `.venv`) | Check that the Rust kernel is installed |
| `rustup component add rust-src` | Add Rust standard-library sources needed by Evcxr |
| `cargo install --locked evcxr_jupyter` | Install the Rust notebook kernel manually |
| `evcxr_jupyter --install` | Register Rust with Jupyter |

**Troubleshooting:** If `cargo` is not found, install Rust and restart your terminal. If `python3` / `py` is not found, install Python 3. If you cannot select Rust, rerun the setup script and check `jupyter kernelspec list`. For full kernel documentation see [Evcxr](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md).

## Contribute

New reproducible AI/research notebooks and simple setup improvements are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[Custom BSD 3-Clause Attribution License](LICENSE) with an additional fourth acknowledgment requirement (not the standard SPDX `BSD-3-Clause`).
