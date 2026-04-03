# Poppins



## Our Mission
- Be the `default` way `developers` create `custom LLMs`



## How much programming is required?
- None! 🥳
- To create an AI w/ Poppins, the only prerequisite is a [train.xml](#what-is-a-trainxml) file & `poppins bootstrap` creates an example one for us!



## Why is Poppins written in Rust?
- **🌍 Deploy Everywhere:**
    - Rust can compile to WebAssembly (`WASM`), allowing AI models to run directly in a web browser
    - Rust can compile to native libraries for `iOS` and `Android`, making it possible to integrate AI into mobile apps
    - B/c Rust does not need a heavy runtime or virtual machine, Rust can run on small devices like the `Raspberry Pi`, so it works optimally in resource constrained environments
- **⚡ Lower Resource Usage:**
    - For the same workload, a Python program consumes more memory and `CPU` than a Rust program b/c Python has interpreter overhead, runtime type checking, and garbage collection, while Rust has none of these
    - Reducing resource usage **lowers costs**
- **🔄 Concurrency:**
    - Python has a Global Interpreter Lock (`GIL`) that prevents multiple threads from running Python code in parallel. This limits CPU usage.
    - Rust allows true parallel execution across all `CPU` cores. This helps AI applications scale efficiently when processing multiple requests or performing compute heavy operations.
- **🔒 File Safety:**
    - Poppins AI models are saved as `.safetensors` files. This format was created by Hugging Face to replace Python’s `pickle` format
    - Python’s `pickle` can execute arbitrary code when loading a file, so malicious model file can compromise a system
    - `.safetensors` files contain no executable code and only contain tensor data. Loading them is safe and does not introduce security risks.



## FAQ's about AI
- https://github.com/beoptimal369/samples/tree/main/ai



## How to create an AI model?
1. Get or Update Rust
    - IF Rust is installed THEN update Rust `rustup update`
    - ELSE install Rust: https://rust-lang.org/learn/get-started/
1. Suggested [VSCode](https://code.visualstudio.com/) / [VSCodium](https://vscodium.com/) extensions:
    - `XML` by Red Hat
    - `rust-analyzer` by rust-lang
    - `Even Better TOML` by tamasfe
1. Create a Project: `cargo new example`
1. Install Poppins: `cargo install poppins`
1. Create a [train.xml](#what-is-a-trainxml): `poppins bootstrap`
1. Update [train.xml](#what-is-a-trainxml) w/ the data you'd love your AI model to be an expert on
1. **(not yet implemented)** Create an AI model: `poppins train`
1. **(not yet implemented)** Ask AI model questions: `poppins infer`



## What is a train.xml?
- To create a custom AI model with Poppins we must define a `train.xml` file
- The `train.xml` tells Poppins what to learn and what configuration settings to use
- To generate an example `train.xml`: `poppins bootstrap`
- A `train.xml` file contains a root `<train>` element with the following sections (only `<samples>` is required. All other sections are optional):
    ```xml
    <train>
        <system-prompts>...</system-prompts>
        <samples>...</samples>
        <prompts>...</prompts>
        <responses>...</responses>
        <sources>...</sources>
        <code-snippets>...</code-snippets>
        <constants>...</constants>
        <phrases>...</phrases>
        <beyond-scope>...</beyond-scope>
    </train>
    ```
    | Section | Description |
    |---------|-------------|
    | `<samples>` | **Required.** Defines the training examples. Each sample references `prompts` & `responses` and may reference `sources` & `code snippets` |
    | `<system-prompts>` | Optional. Define AI system prompts, identified by an `id` |
    | `<prompts>` | Optional. Reusable prompts (questions), identified by an `id` |
    | `<responses>` | Optional. Reusable ai responses (answers), identified by an `id` |
    | `<sources>` | Optional. References to external sources (URLs, titles) identified by an `id` |
    | `<code-snippets>` | Optional. Reusable code blocks in specific languages, identified by an `id` |
    | `<constants>` | Optional. Training hyperparameters (`val_interval`, `aim_loss`, `aim_train_gb` etc.) |
    | `<phrases>` | Optional. Patterns with variant values for data augmentation |
    | `<beyond-scope>` | Optional. Defines topics that are beyond the scope of the AI's knowledge to auto-generate samples teaching the AI how to respond w/ a custom "I don't know" response |



## Plan to 1.0
- ✅ Create **fundamentals** for `Ternary Quantization` based on `BitNet` research:
    - ✅ https://arxiv.org/pdf/2310.11453v1
    - ✅ https://arxiv.org/pdf/2402.17764v1
    - ✅ https://arxiv.org/pdf/2411.04965v1
    - ✅ https://arxiv.org/pdf/2504.12285
- ✅ Stub Poppins front doors
    - ✅ `bootstrap()`: Will create example `train.xml`
    - ✅ `train()`: Will create model based on `train.xml`
    - ✅ `infer()`: Will get response from model
    - ✅ `poppins bootstrap`: CLI command that calls `bootstrap()`
    - ✅ `poppins train`: CLI command that calls `train()`
    - ✅ `poppins infer`: CLI command that calls `infer()`
- ✅ Push to [GitHub](https://github.com/beoptimal369/poppins)
- ✅ Push to [crates.io](https://crates.io/crates/poppins)
- ✅ Deploy [`train.xsd` to a Cloudflare Worker](https://xsd.beoptimal369.workers.dev/?version=0.1.0)
- ✅ `bootstrap()`
    - ✅ Accept an `output_dir_path` default to `cwd` & write example `train.xml`
    - ✅ May also be called via cli @ `poppins bootstrap`
    - ✅ CLI accepts `-o` or `--output` params for `output_dir_path`
- ✅ `BPETokenizer`
    - ✅ Write `tokenizer.json` based on `train.xml` samples
    - ✅ Add `bpe_requested_tokens` to `train.xml` constants
    - ✅ Add `bpe_min_merge_frequency` to `train.xml` constants
- `train()`:
    - ✅ Read training file (default to `train.xml`)
    - ✅ Parse `train.xml`
    - ✅ Validate `train.xml`
    - ✅ Create `TrainXML`
    - ✅ Write output directory (default to `.poppins`)
    - ✅ Create `Samples` (holds `training` & `validation` `samples`)
    - ✅ Write `output_dir/train_corpus.txt`
    - ✅ Write `output_dir/val_corpus.txt`
    - ✅ Write `output_dir/tokenizer.json`
    - ✅ Write `output_dir/train_corpus.bin`
    - ✅ Write `output_dir/val_corpus.bin`
    - ✅ Write `output_dir/train_index.bin`
    - ✅ Write `output_dir/val_index.bin`
    - Write `output_dir/manifest.json`
- ...
- MLA
    - https://arxiv.org/pdf/2602.10718
- RMSNorm
- RoPE
- ReLU²
- KV Cache
- Save conversations to file
    - Multi Turn
    - RLM
- Add files to context
    - Abstract Syntax Tree
- Add images to samples / context
