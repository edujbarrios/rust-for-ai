//! Load real pretrained CNN weights and set up transfer/fine-tuning in tch.
//! No training is performed. This uses an adapter on the pretrained 1000 logits
//! (not a replacement of the original feature classifier).
use std::error::Error;
use std::path::PathBuf;
use tch::{nn, vision::{mobilenet, resnet, vgg}, Device};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: cargo run -- <resnet18|mobilenet-v2|vgg19> <head|deep|full> <num_classes>".into());
    }
    let model_name = args[1].as_str();
    let mode = args[2].as_str();
    if !["head", "deep", "full"].contains(&mode) {
        return Err("mode must be head, deep or full".into());
    }
    let num_classes: i64 = args[3].parse()?;
    if num_classes < 2 {
        return Err("num_classes must be at least 2".into());
    }
    let (file, last_block): (&str, &str) = match model_name {
        "resnet18" => ("resnet18.ot", "layer4"),
        "mobilenet-v2" => ("mobilenet-v2.ot", "features.17"),
        "vgg19" => ("vgg19.ot", "features.34"),
        _ => return Err("model must be resnet18, mobilenet-v2 or vgg19".into()),
    };
    let cache: PathBuf = std::env::temp_dir().join("rust-for-ai-cnn-weights").join(file);
    if !cache.exists() {
        return Err(format!("missing {}: run notebook 04 first", cache.display()).into());
    }

    let mut vs = nn::VarStore::new(Device::Cpu);
    // Build with 1000 ImageNet outputs to match the published .ot checkpoints.
    let net: Box<dyn nn::ModuleT> = match model_name {
        "resnet18" => Box::new(resnet::resnet18(&vs.root(), 1000)),
        "mobilenet-v2" => Box::new(mobilenet::v2(&vs.root(), 1000)),
        "vgg19" => Box::new(vgg::vgg19(&vs.root(), 1000)),
        _ => unreachable!(),
    };
    vs.load(&cache)?;
    println!("Loaded {model_name} from {}", cache.display());

    // Freeze pre-trained tensors BEFORE creating the new adapter head.
    vs.freeze();
    let adapter = nn::linear(&vs.root() / "transfer_head", 1000, num_classes, Default::default());

    match mode {
        "head" => { /* Only the newly created adapter is trainable. */ }
        "deep" => {
            let mut unfrozen = 0usize;
            // Names are implementation-specific. Fail rather than silently
            // claiming a partial fine-tuning if the prefix doesn't match.
            for (name, tensor) in vs.variables() {
                if name.contains(last_block) {
                    let _ = tensor.set_requires_grad(true);
                    unfrozen += 1;
                }
            }
            if unfrozen == 0 {
                return Err(format!("no backbone variables match '{last_block}'; inspect vs.variables() names").into());
            }
        }
        "full" => vs.unfreeze(),
        _ => unreachable!(),
    }

    let named = vs.variables();
    let trainable = named.values().filter(|v| v.requires_grad()).count();
    println!("mode={mode}; classes={num_classes}; tensors total={}; trainable={trainable}", named.len());
    println!("New 1000 -> {num_classes} trainable adapter on ImageNet logits.");
    println!("No optimizer step or training performed.");
    // net and adapter are retained: a later training loop could compute
    // let logits = image_batch.apply_t(&*net, true).apply(&adapter);
    // let loss = logits.cross_entropy_for_logits(&labels);
    // optimizer.backward_step(&loss);
    let _ = (&net, &adapter);
    Ok(())
}
