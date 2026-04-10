# Poppins


[![Crates.io](https://img.shields.io/crates/v/poppins.svg)](https://crates.io/crates/poppins)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75+-blue.svg)](https://rust-lang.org)


## What is Poppins?
- `Create AI` from scratch, with `no programming` required!


## 🧐 How?
1. Define FAQs (example `prompts` and example `responses`) that you'd love your AI to learn
2. Your AI learns from these examples (`training`)
3. Your AI accepts prompts and provides responses (`inference`)!


## Why create AI?
- **🪄 Easy**
    - No programming required
    - Just provide FAQs (`samples`)
- **🔒 Private**
    - No internet access required
    - Your AI, your prompts & your responses, remain on your machine
    - The intelligence is on your machine!
- **💰 Free**
    - No usage limits
    - No subscription fees
    - Poppins is Open Source, so the AI models you create AND the AI models Poppins provides are all 100% yours!


## Poppins vs Cloud AI
- When we send a prompt to Cloud AI (ex: `ChatGPT` / `Anthropic` / `Gemini`):
    1. Our question travels over the internet to a massive data center
    2. The data center computes a response
    3. We receive the response
- With Cloud AI there is no privacy b/c our conversations train their models
- But with Poppins you don't need internet, your computer does the thinking & your conversations remain private
    | Category| Poppins | Cloud AI |
    |---|:----------|:---------|
    | **Internet Required** | 🟢 No | 🔴 Yes |
    | **Usage Limits** | 🟢 No | 🔴 Yes |
    | **Privacy** | 🟢 Yes| 🔴 No |


## Can I create AI on my computer?
- ✅ Yes!
- We've designed Poppins to be as efficient as possible
    - Your AI can be created on any laptop (`training`)
    - Your AI can accept prompts and provide responses on any laptop or phone (`inference`)


## 🤔 How is Poppins efficient?
- Most AI models need powerful data center GPUs
- Poppins uses a different approach called `ternary quantization`
- Instead of storing numbers with high precision Poppins stores them as `-1`, `0`, or `+1`


## Does ternary quantization make the AI less intelligent?
- No, b/c our intelligence does not come from the precision of numbers. 😊 Our intelligence comes from:
    1. 🔗 **Connection:** We use more connections than typical models, so the simplicity of each individual number is offset by having many more of them working together.
    2. 🎯 **Focus:** We pay attention to the most relevant parts of the conversation rather than trying to remember everything equally, similar to how people naturally read and listen.
    3. 📦 **Compression:** We compress memory without losing meaning, to retain important information with less space required.


## Who is Poppins for?
- 🎓 **Students**: Learn AI concepts with ease
- 🏢 **Businesses**: Keep customer data private
- 👩‍💻 **Developers**: Add AI to your app without API costs
- 🏠 **Hobbyists**: Create specialized AI for your interests


## Our Mission
- **🔒 Privacy**
    - Poppins never uses your prompts or conversations to train our AI models
    - What happens on your machine stays on your machine
- **📖 Transparency**
    - Every line of Poppins code is public
    - Every training sample our models train w/ is public
- **🤝 Human in the Loop**
    - Our models read files but never edit files
    - Any code our AI's generate is provided for a human to copy, and paste
- **⚡️ Be Optimal**
    - Efficient Training: Train on any laptop
    - Efficient Inference: Infer on any smartphone
    - Quality Responses: Be a benchmark leader


## Our RoadMap
- **Poppins 1 (`Version 1.0.0`)**
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
    - `bootstrap()`
        - ✅ Write `train.xsd`
        - ✅ Write `train.xml`
        - ✅ Write `english.xml`
        - ✅ Write `math.xml`
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
        - Forward Pass
            - KV Cache
            - RoPE
            - ReLU²
            - RMSNorm
            - SnapMLA
                - https://arxiv.org/pdf/2602.10718
        - Backwards Pass
            - Gradient Computation
            - AdamW
        - Checkpoint Saves
            - Resume from checkpoint
        - Train on Intel
        - Train on Kaggle
    - `infer()`:
        - Temperature
        - Quality Responses
    - `beoptimal.org/poppins`
        - Demo AI model that knows english, math & one other niche topic
- **Poppins 2 (`Version 2.0.0`)**
    - `bootstrap()`
        - Write `ai.xml`
        - Write `rust.xml`
        - Write `xml.xml`
        - Write `json.xml`
        - Write `markdown.xml`
        - Write `bash.xml`
        - Write `poppins.xml`
    - `beoptimal.org/poppins` & `Desktop App`
        - Provide free/local AI model that knows:
            - `English`
            - `Math`
            - `AI`
            - `Rust`
            - `XML`
            - `JSON`
            - `Markdown`
            - `Bash`
            - `Poppins`
    - `Desktop App`
        - Update `train.xml` w/o needing to open xml file
- **Poppins 3**
    - `beoptimal.org/poppins` & `Desktop App`
        - Browse / Import community `train.xml` files
        - Conversation UI:
            - Multi Turn
            - RLM
            - Turso DB (option to sync w/ Turso cloud for multi device syncing)
    - Quality Documentation
- **Poppins 4**
    - `bootstrap()`
        - Write `html.xml`
        - Write `css.xml`
        - Write `js.xml`
        - Write `ts.xml`
        - Write `tsx.xml`
        - Write `git.xml`
        - Write `sql.xml`
        - Write `drizzle.xml`
        - Write `tauri.xml`
        - Write `ace.xml`
        - Write `solid.xml`
    - Add content from local files to samples / context
        - Abstract Syntax Tree
    - Quality Benchmark Scores
- **Poppins 5**
    - MCP
    - SKILLS.md
    - URL Tool Call
    - Add images to samples / context
        - Text images (error messages)
- **Poppins 6**
    - Prompt to Design
    - Prompt to Edit Design
    - Design to Code
    - Screen Recording to Code
    - Provide **free/local** model for `Design`, `Web`, `App` & `AI` development 


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


## How to create an AI model?
1. Get or Update Rust
    - IF Rust is installed THEN update Rust
        - bash: `rustup update`
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
1. **(under construction)** Create an AI model: `poppins train`
1. **(under construction)** Ask AI model questions: `poppins infer`


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
    | `<imports>` | Lets us import a local training xml files into this one |
    | `<samples>` | Defines the training examples. Each sample references `<prompts>` & `<responses>` and may reference`<system-prompts>`, `<thoughts>`, `<sources>` & `<code-snippets>` |
    | `<system-prompts>` | Define AI system prompts, identified by an `id` |
    | `<prompts>` | Reusable prompts (questions), identified by an `id` |
    | `<thoughts>` | Reusable thoughts, identified by an `id` that help teach the model how to compute an optimal response |
    | `<responses>` | Reusable ai responses (answers), identified by an `id` |
    | `<sources>` | References to external sources (URLs, titles) identified by an `id` |
    | `<code-snippets>` | Reusable code blocks in specific languages, identified by an `id` |
    | `<constants>` | Training hyperparameters (`val_interval`, `aim_loss`, `aim_train_gb` etc.) |
    | `<phrases>` | Patterns with variant values for data augmentation |
    | `<beyond-scope>` | Defines topics that are beyond the scope of the AI's knowledge to auto-generate samples teaching the AI how to respond w/ a custom "I don't know" response |


## FAQ's about AI
- https://github.com/beoptimal369/samples/tree/main/ai
