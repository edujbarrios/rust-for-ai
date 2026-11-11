# CNN transfer learning with tch-rs

First run [notebook 04](../../notebooks/04_pretrained_cnns.ipynb) to download
ResNet18, MobileNetV2 and VGG19 `.ot` checkpoints. They are stored in
`std::env::temp_dir()/rust-for-ai-cnn-weights/`, not in git.

This Rust CLI demonstrates actual `tch` model construction, checkpoint loading,
freeze/unfreeze and an added **logits adapter** (1000 ImageNet scores to your own
number of classes). It does **not** replace the backbone's original head, train,
or claim a meaningful new-task accuracy. For higher quality transfer learning,
extract penultimate activations and replace the original classifier instead.

Requires a compatible libtorch/PyTorch installation. Consult
[the official tch README](https://github.com/LaurentMazare/tch-rs#readme)
for **version-matching setup** before running:

```bash
export LIBTORCH_USE_PYTORCH=1   # only if a compatible PyTorch is installed
cargo run --manifest-path examples/cnn_finetuning/Cargo.toml -- resnet18 head 3
cargo run --manifest-path examples/cnn_finetuning/Cargo.toml -- mobilenet-v2 deep 3
cargo run --manifest-path examples/cnn_finetuning/Cargo.toml -- vgg19 full 3
```

Because VGG19 is large, allow sufficient RAM. No dataset, training loop, or
backpropagation is run. This optional binary isn't part of the default standard
library-only CI checks.
