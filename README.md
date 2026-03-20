# Poppins


## Our Mission
- Be the `default` way `developers` create `custom LLMs`


## Plan to 1.0
- ✅ Create **fundamentals** for `Ternary Quantization` based on `BitNet` research:
    - https://arxiv.org/pdf/2310.11453v1
    - https://arxiv.org/pdf/2402.17764v1
    - https://arxiv.org/pdf/2411.04965v1
    - https://arxiv.org/pdf/2504.12285
- ✅ Create API front doors
    - `bootstrap()`: `println!("Will write to cwd a simple train.xml")`
    - `train()`: `println!("Will train / create a model based on a train.xml")`
    - `infer()`: `println!("Will send a prompt to an Ai model and provide the response")`
    - `poppins bootstrap`: CLI command that calls `bootstrap()`
    - `poppins train`: CLI command that calls `train()`
    - `poppins infer`: CLI command that calls `infer()`
- ✅ Push to [GitHub](https://github.com/beoptimal369/poppins)
- ✅ Push to [crates.io](https://crates.io/crates/poppins)
- ✅ Deploy [`train.xsd` to a Cloudflare Worker](https://xsd.beoptimal369.workers.dev/?version=0.1.0)
- `train()`:
    - ✅ Read training file (default to `train.xml`)
    - ✅ Parse `train.xml`
    - ✅ Validate `train.xml`
    - ✅ Create `TrainXML`
    - ✅ Write output directory (default to `.poppins`)
    - ✅ Create `Samples` (holds `training` & `validation` `samples`)
    - ✅ Write `output_dir/train_corpus.xml`
    - ✅ Write `output_dir/val_corpus.xml`
    - Write `output_dir/vocab.json`
    - Write `output_dir/manifest.json`
- ...
- Heap's Law
- BPE
- MLA
- Swiglu
- RMS Norm
- Memory
    - Multi Turn
    - Turso
    - RLM
    - Abstract Syntax Tree
