# Poppins



## Our Mission
- Be the `default` way `developers` create `custom LLMs`



## How much programming is required?
- None! 🥳
- To create an AI w/ the Poppins CLI, the only prerequisite is a [train.xml](#what-is-a-trainxml) file
- Poppins may also can be called from w/in Rust code @ `bootstrap()`, `train()` & `infer()`



## Why is Poppins written in Rust?
- **🌍 Deploy Everywhere:**
    - Rust compiles to WebAssembly (`WASM`), allowing AI models to run directly in web browsers
    - Rust compiles to native libraries for `iOS` and `Android`, making it possible to integrate AI into mobile apps
    - B/c Rust does not need a heavy runtime or virtual machine, Rust can run on small devices like the `Raspberry Pi`, making it ideal in resource constrained environments
- **⚡ Lower Resource Usage:**
    - For the same workload, a Python program consumes more memory and `CPU` than a Rust program b/c Python has interpreter overhead, runtime type checking, and garbage collection, while Rust has none of these & reducing resource usage **lowers costs**
- **🔄 Concurrency:**
    - Rust allows parallel execution across `CPU` cores & `GPU` devices which helps us scale efficiently
- **🔒 File Safety:**
    - Poppins AI models are saved as `.safetensors` files. This format was created by Hugging Face to replace Python’s `pickle` format
    - Python’s `pickle` can execute arbitrary code when loading a file, so malicious model file could compromise a system
    - `.safetensors` files contain no executable code and only contain tensor data. Loading them is safe and introduces no security risks.



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
1. Create a Project
    - bash: `cargo new example`
1. Install Poppins
    - bash: `cargo install poppins`
1. Create a [train.xml](#what-is-a-trainxml)
    - bash: `poppins bootstrap <model_name>`
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



## 1.0 Plan
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
    - ✅ Write `train.xml`
        - hyperparams autocomplete
        - import w/in `train.xml `files
    - ✅ Write `train.xsd`
- `train()`:
    - ✅ Parse `train.xml`
    - ✅ Validate `train.xml`
    - ✅ Create `TrainXML`
    - ✅ Write `output_dir/train_corpus.txt`
    - ✅ Write `output_dir/val_corpus.txt`
    - ✅ Write `output_dir/tokenizer.json`
    - ✅ Write `output_dir/train_corpus.bin`
    - ✅ Write `output_dir/val_corpus.bin`
    - ✅ Write `output_dir/train_index.bin`
    - ✅ Write `output_dir/val_index.bin`
    - ✅ Write `output_dir/config_poppins.json`
    - ✅ Write `output_dir/config.json`
    - ✅ Device (`cuda` / `metal` / `cpu`) detection / selection
    - Per Token Quantization
    - Forward Pass
        - KV Cache
        - RoPE
        - ReLU²
        - RMSNorm
        - SnapMLA
            - https://arxiv.org/pdf/2602.10718
    - Backwards Pass
    - Checkpoint Saves
- `infer()`:
    - Temperature
    - Quality Responses
- Provide Free AI that knows `English`, `Math`, `Rust`, `XML`, `JSON` & `Poppins`
- Provide comprehensive Poppins Documentation



## 2.0 Plan
- UI
- Save conversations to local db
    - Multi Turn
    - RLM
- Add files to context
    - Abstract Syntax Tree
- Add images to samples / context
