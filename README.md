# Poppins

## Our Mission
- Be the `default` way `developers` create `custom LLMs`

## Mission Fundamentals
- **Efficiency**
    - Run optimally on all device
- **Simplicity**
    - Small API surface
    - Only prerequisite is one `train.xml` file

## Plan to 1.0
- ✅ Create **fundamentals** for `Ternary Quantization` based on `BitNet` research:
    - https://arxiv.org/pdf/2310.11453v1
    - https://arxiv.org/pdf/2402.17764v1
    - https://arxiv.org/pdf/2411.04965v1
    - https://arxiv.org/pdf/2504.12285
- Create the front doors (only `println!()`)
    - `help()`
        - `println!("Will provide helpful info about how to use poppins")`
    - `bootstrap()`
        - `println!("Will write to cwd a simple train.xml")`
    - `train()`
        - `println!("Will train / create a model based on the train.xml")`
    - `infer()`
        - `println!("Will send a prompt to an Ai model and provide the response")`
    - `poppins help`
        - Calls `help()` from CLI
    - `poppins bootstrap`
        - Calls `bootstrap()` from CLI
    - `poppins train`
        - Calls `train()` from CLI
    - `poppins infer`
        - Calls `infer()` from CLI
- Push to GitHub
- Push to crates.io
- Deploy `train.xsd` to Cloudflare Worker
- `train()`:
    - Reads `train.xml` & then creates `TrainXML`
    - Writes `.poppins/vocab.json`
    - Writes `.poppins/manifest.json`
- ...
-  Be the `default` way `developers` create `custom LLMs`
