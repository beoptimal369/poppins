# Poppins


## Our Mission
- Be the `default` way `developers` create `custom LLMs`


## Why is Poppins written in Rust?
- **📦 Single Binary Deployment:**
    - Python applications require a Python interpreter and installed dependencies to run. Deploying to production often involves virtual environments, `Docker` containers and managing system packages.
    - Rust compiles to a `single executable binary`. This binary contains all the code and dependencies. To deploy we just copy the binary to the target machine and run it. There is no need to install a runtime or manage dependencies. This simplifies deployment and reduces configuration errors.
- **🌍 Deploy Everywhere:**
    - Rust can compile to WebAssembly (`WASM`), allowing AI models to run directly in a web browser
    - Rust can compile to native libraries for `iOS` and `Android`, making it possible to integrate AI into mobile apps
    - B/c Rust does not need a heavy runtime or virtual machine, Rust can run on small devices like the `Raspberry Pi`, so it works optimally in resource constrained environments
- **⚡ No Runtime Overhead from Dynamic Types:**
    - Python is a `dynamically` typed language. Every time a Python program runs an operation, it must check and resolve the types of values at runtime. This adds overhead to every instruction.
    - Rust uses `static` typing. All types are known at compile time, so there is no type checking overhead during execution. This makes Rust significantly faster for the compute heavy operations that are common w/ AI.
- **🧠 Lower Resource Usage:**
    - For the same workload, a Python program consumes more memory and `CPU` than a Rust program. Python has interpreter overhead, runtime type checking, and garbage collection, while Rust has none of these
    - This is especially important for AI workloads on edge devices, mobile phones, or servers where reducing resource usage **lowers costs**
- **📊 Consistent Performance:**
    - Languages like Python, Java, and JavaScript use a Garbage Collector (`GC`) to manage memory. The GC can pause the program at random times to clean up memory, which can cause delays during AI model responses.
    - Rust does not use a Garbage Collector. Memory is managed at compile time, so there are no unexpected pauses during inference. This keeps response times consistent.
- **🚀 No Cold Start Delays:**
    - In `serverless` or containerized environments, Python applications can have slow startup times because the interpreter must load and initialize dependencies.
    - Rust binaries start instantly. There is no interpreter initialization. For AI applications that need to scale up quickly or run in `serverless` functions, Rust eliminates cold start latency.
- **🔄 Concurrency:**
    - Python has a Global Interpreter Lock (`GIL`) that prevents multiple threads from running Python code in parallel. This limits CPU usage.
    - Rust allows true parallel execution across all `CPU` cores. This helps AI applications scale efficiently when processing multiple requests or performing compute heavy operations.
- **🔒 File Safety:**
    - Poppins AI models are often saved as `.safetensors` files. This format was created by Hugging Face to replace Python’s `pickle` format
    - Python’s `pickle` can execute arbitrary code when loading a file. A malicious model file could compromise a system.
    - `.safetensors` files contain only tensor data, no executable code. Loading them is safe and does not introduce security risks.


## FAQ's about AI
- https://github.com/beoptimal369/samples/tree/main/ai



## How to create an AI model?
1. Get or Update Rust
    - IF Rust is installed THEN update Rust `rustup update`
    - ELSE install Rust: https://rust-lang.org/learn/get-started/
1. Create a Project: `cargo new example`
1. Install Poppins: `cargo install poppins`
1. Create a `train.xml`: `poppins bootstrap`
1. Update `train.xml` w/ the data you'd love your AI model to be an expert on
1. **(not yet implemented)** Create an AI model: `poppins train`
1. **(not yet implemented)** Ask AI model questions: `poppins infer`


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
    - ✅ Write `output_dir/train_corpus.xml`
    - ✅ Write `output_dir/val_corpus.xml`
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
- Memory
    - Multi Turn
    - Turso
    - RLM
    - Abstract Syntax Tree
