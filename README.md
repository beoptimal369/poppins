# Poppins


## Our Mission
- Be the `default` way `developers` create `custom LLMs`


## Plan to 1.0
- ✅ Create **fundamentals** for `Ternary Quantization` based on `BitNet` research:
    - https://arxiv.org/pdf/2310.11453v1
    - https://arxiv.org/pdf/2402.17764v1
    - https://arxiv.org/pdf/2411.04965v1
    - https://arxiv.org/pdf/2504.12285
- ✅ Stub Poppins front doors
    - `bootstrap()`: Will create example `train.xml`
    - `train()`: Will create model based on `train.xml`
    - `infer()`: Will get response from model
    - `poppins bootstrap`: CLI command that calls `bootstrap()`
    - `poppins train`: CLI command that calls `train()`
    - `poppins infer`: CLI command that calls `infer()`
- ✅ Push to [GitHub](https://github.com/beoptimal369/poppins)
- ✅ Push to [crates.io](https://crates.io/crates/poppins)
- ✅ Deploy [`train.xsd` to a Cloudflare Worker](https://xsd.beoptimal369.workers.dev/?version=0.1.0)
- ✅ Implement `bootstrap()` & `poppins bootstrap`
- `train()`:
    - ✅ Read training file (default to `train.xml`)
    - ✅ Parse `train.xml`
    - ✅ Validate `train.xml`
    - ✅ Create `TrainXML`
    - ✅ Write output directory (default to `.poppins`)
    - ✅ Create `Samples` (holds `training` & `validation` `samples`)
    - ✅ Write `output_dir/train_corpus.xml`
    - ✅ Write `output_dir/val_corpus.xml`
    - ✅ Write `output_dir/tokenizer.json` (BPE)
    - Write `output_dir/train_corpus.bin`
    - Write `output_dir/val_corpus.bin`
    - Write `output_dir/vocab.json`
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


## FAQ

### What is a neural network?
- A neural network is a mathematical function that transforms an input into an output through a series of calculations
- A neural network's mathematical function includes weights and biases that are used to calculate the output
- At the beginning of training the weights and biases are random & through training these numbers get good enough to produce quality outputs


### What is a model?
- A model is an instance of a neural network that has been trained w/ samples, can recieve inputs (prompts) and provides quality outputs (responses)


### What is training?
- Training is the process of creating a model that makes useful next token predictions
- In the training process we show the model samples and let it learn from its mistakes (adjust its weights and biases)


### What is a sample?
- A sample is a simple training example that includes atleast 1 prompt and 1 model response
- A sample may also include code snippets and sources


### What is a multi-turn sample?
- A multi-turn sample is a sample w/ multiple prompts and responses, to teach the model how to:
    - Have a conversation
    - Ask good follow up questions
    - Build on previous responses


### What is a corpus?
- A corpus is a collection of samples


### What is a hidden state?
- Math anotation is `h`
- The hidden state is the "current understanding" of the input as it flows through the model
- Input tokens start as embeddings (not yet hidden states)
- After passing through the first transformer layer, they become hidden states
- Each layer transforms the hidden state further
- A hidden state is a token vector after it has passed through atleast one layers
- Hidden b/c
    - Internal
    - Not directly visible
    - Intermediate representations
- Identifies what the model “knows” about the sequence


### What is the final hidden state?
- The final hidden state (last layer's output) is what gets multiplied by output weights to predict the next token


### What is a weight?
- Updated during training
- Fixed during inference
- Weights are the learned parameters that transform inputs
- Raw weights are `f32`
- Quantized weights are `-1`, `0` or `1`


### What is an Output Projection Vector?
- Annotation: `W_out[i]`
- An Output Projection Vector is a vector of weights of `embedding_dim` length for a token that identifies what hidden state pattern predicts this token
- Each token w/in the model's vocabulary has an Output Projection Vector
- When we multiply the Output Projection Vector with the hidden state, we get a score indicating how well the hidden state matches the token


### What is an Output Projection Matrix?
- Annotation: `W_out`
- Output Projection Matrix is `embedding_dim` length and `vocab_size` height (`[vocab_size, hidden_dim]`)


### What is a Linear Layer Row?
- A Linear Layer Row is a vector of `input_dim` length that identifies what hidden state pattern predicts this neuron
- Each neuron w/in a layer has a Linear Layer Row
- When we multiply the Linear Layer Row with the hidden state, we get a score indicating how well the hidden state matches the neuron


### What is a Linear Layer Matrix?
- A Linear Layer Matrix is `input_dim` length and `output_dim` height (`[output_dim, input_dim]`)


### What is a bias?
- A bias is single number added during the output calculation
- Fixed during training
- The bias is a constant number added after the weighted sum
- Each token has bias and each neuron has a bias
- Tells us How likely a token / neuron is in general
- Small bias -> token / neuron rarely appears
- High bias -> token / neuron appears often in many contexts


### What is the output bias?
- An output bias is computed during training, is a unique value for each token and identifies baseline tendencies for a token
- High bias -> token appears often in many contexts
- Small bias -> token rarely appears


### What is inference?
- Inference is when we use a trained model to generate a response


### What is a token?
- A token is a piece of text that the model understands as a single unit
- Tokens can be:
    - Words
    - Parts of words
    - Punctuation
    - Individual characters
- Spaces are typically attached to the following word & not separate tokens


### What is a tokenizer?
- A tokenizer is a tool that converts text into numbers (and back)
- Computers don't understand words like "hi" - they only understand numbers
- A tokenizer finds the middle ground: - it 
    - IF we give every word a unique number THEN we need a very large dictionary & can't handle words we've never seen
    - IF we give every character a unique number THEN we lose word meanings
    - Tokenizers splits text into pieces called "tokens" and gives each token a unique number ID


### What is BPE?
- BPE stands for Byte Pair Encoding
- BPE is a method for deciding how to split text into tokens
- BPE learns from the corpus which character & token combinations appear most frequently together, then merges them into tokens


### How does BPE work?
- Start with individual characters, spaces and punctuations marks as separate tokens
- Count how often each adjacent pair of tokens appears next to each other in the entire corpus
- Find the most frequent pair
- IF the most frequent pair occurs more then MIN_MERGE_FREQUENCY (ex: 3) times THEN merge them into a new token and repeat the process ELSE stop merging


### What are merge rules?
- Merge rules tell us how to build bigger tokens from smaller ones
- When we get NEW text (not in the training data) (like a user prompt), we apply the merge rules to tokenize the text


### How are merge rules used?
- Split prompt into characters, spaces and punctuation marks
- Apply merge rules in the exact same order they were learned to build tokens & ensure consistency
- Look up each token in the vocabulary to get its token ID
- Look up each token embedding based on the token ID


### What is embedding?
- Embedding is the process of turning a token into a token embedding


### What is a token embedding?
- A token embedding is a vector of numbers that represents the meaning of a token


### What is a vector?
- A vector is a list of numbers (ex: `[1.5, 0, -2.3]`)


### What is embedding dimension?
- Embedding dimension is the length of a token embedding vector
- More dimensions = more expressive power = more memory and computation


### What is a dimension?
- Slot / Index / Position w/in a vector


### What is the origin?
- Where x, y & z meet


### What is a basis axis?
- A basis axis is a unique direction from the origin that aligns w/ a dimension
- Unique meaning no 2 basis axis w/in a vector share the same orientation
- Models distribute meaning (nouniness, verbiness, pronouniness, animalness) across dimensions (basis axes)
- What each dimension represents is not human defined, only the number of allowed dimensions (embedding dimension) is human defined


### What is a latent feature?
- A latent feature is a pattern the model discovered during training that:
    - Is not explicitly named
    - We did not manually define
    - Exists only as numbers inside the network
- Each dimension w/in a vector captures a pattern in the data, we do not know what that pattern is and there is no guarantee that it corresponds to a clean human concept (ex: animalness)
- The model discovers useful internal dimensions automatically
- Every dimension in embeddings, hidden states & neuron outputs is a latent feature



### What is orientation?
- Orientation is what way an arrow points from the origin
- Independent of magnitude


### What is an input?
- An input is the token embeddings for all tokens w/in a sequence
- A model moves the input through layers to comprehend the input & then predict the next token
- An input is a matrix of numbers (length = embedding dimension, height = token length) that comes from somewhere, that somewhere might be the:
    - Original sequence
    - Output of a previous layer


### What is a sequence?
- A sequence is an ordered list of tokens


### What is a matrix?
- A matrix is a rectangular grid of numbers with rows and columns


### What is a weighted input?
- x1 * w1
- h1 * w1
- A weighted input is the result of multiplying an input by its corresponding weight


### What is an output?
- An output is a vector that is provided by a layer after aligning inputs w/ ternary weights
- The output size is equal to the number of neurons in a layer
- During attention & ffn compress the output size is equal to the embedding dimension
- During ffn expand the output size is equal to the embedding dimension * 4


### Got an output calculation example?
```txt
input = [2.0, 1.5, 0.5]

weights = [
    [1, 0, -1],   // neuron 0
    [0, 1, -1],   // neuron 1
    [-1, 1, -1],  // neuron 2
]

output[0] = (2.0×1) + (1.5×0) + (0.5×-1) = 2.0 + 0.0 - 0.5 = 1.5
output[1] = (2.0×0) + (1.5×1) + (0.5×-1) = 0.0 + 1.5 - 0.5 = 1.0
output[2] = (2.0×-1) + (1.5×1) + (0.5×-1) = -2.0 + 1.5 - 0.5 = -1.0

output = [1.5, 1.0, -1.0]  // length 3
```


### What is a layer?
- Collection of neurons that process data simultaneously
- Each token goes through each neuron in a layer
- The number of neurons w/in a layer is equal to the output_size


### What is Quantization?
- 32-bit floating point numbers (`f32`) can represent most numbers with high precision, example:
    - `0.000000001`
    - `3.1415926535`
    - `-0.999999999`
- Precise (`f32`) numbers take up 4 bytes (32 bits) for every value
- Quantization is the process of taking numbers that need many bits (`3.1415926535`)and mapping them to numbers that need fewer bits (`3.14`)


### What is Ternary?
- Ternary means 3 possible values


### What is Ternary Quantization?
- Ternary Quantizaion is a technique where we keep raw weights in `f32` & create quantized weights that can be `-1`, `0` or `+1`
- Raw values require 32 bits but quantized values (what we'll use in inference) only require 2 bits to tells us the value (`0b01` tells us `-1`, `0b00` tells us `0` & `0b10` tells us `+1`)


### Why the 0b in bit encoding?
- `0b` is a prefix that tells the computer "there are binary digits coming next"


### Why does 0b01 equal -1?
- Not an outside of Poppins rule, just a choice, the on and off bits can represent whatever values we want them to be


### A weight of +1 means?
- This input matters a lot, add its effect


### A weight of 0 means?
- Ignore this input entirely


### A weight of -1 means?
- This input matters, but in the OPPOSITE direction


### What is a neuron?
- A neuron is a function that learns to detect different features (patterns)
- This function has 1 weight vector and 1 bias number
- A neurons job is to accept an embedding vector and provide a number that represents this tokens score as it relates to a particular feature
- Calculation is dot product between input & weights
- Each neuron learns a different pattern & together they form a massive pattern-detection system


### How is Ternary Quantization done?


### What is absolute value?
- The absolute value of a number is its distance from zero, ignoring whether it's positive or negative


### What is a logit?
- `dot product + bias`
- `logit = (weight₁ * input₁) + (weight₂ * input₂) + ...  + (weightₙ * inputₙ) + bias`
- When predicting the next token, the model computes logits for all tokens w/in its vocabulary
- Larger logits mean the model thinks that token is more likely
- Logits aren't probabilities (they don't sum to 1 & they can be negative)
- Logits tells us, how compatible a token is w/ the current hidden state


### What is softmax?
- Softmax is the process of converting logits to probabilities
- `probability = eulers_num^(logit) / sum(eulers_num^(all_logits))`
- Numerator is Euler's number raised to the logit for one token
- Denomenator is the sum of Euler's number raised to the logit for all tokens
- Example:
    ```txt
    Vocabulary is 3 tokens
    Token 0: "apple"
    Token 1: "banana"  
    Token 2: "cherry"

    logits = [2.0, 1.0, 0.1]

    apple_numerator = e^(2.0) = 7.389
    banana_numerator = e^(1.0) = 2.718
    cherry_numerator = e^(0.1) = 1.105

    denominator = sum(e^(all_logits)) = 7.389 + 2.718 + 1.105 = 11.212
    ```


### What is Euler's number?
- Euler's number, approximately 2.71828 is a mathematical constant like `π`
- In neural networks, euler's number appears in the softmax formula because euler's number:
    - Raised to any power is always positive (no negative numbers)
    - Grows exponentially (large logits become much larger, small logits become tiny)


### Got Euler's number examples?
```txt
e^0 = 1
e^1 = 2.718
e^2 = 7.389
e^3 = 20.085
e^(-1) = 0.368
e^(-2) = 0.135
```


### What is SwiGLU?
- SwiGLU stands for Swish Gated Linear Unit
- SwiGLU is an activation function used in feed-forward networks


### What is an activation function?
- An activation function transforms a logit into an output


### What is the ReLU?
- ReLU stands for Rectified Linear Unit
- ReLU is an activation function
- IF `logit > 0` THEN `output = logit` ELSE `output = 0`
- `output = ReLU(logit) = max(0, logit)`


### What is the ReLU?
- ReLU stands for Rectified Linear Unit
- ReLU² stands for ReLU squared
- ReLU is an activation function
- Raise the ReLU to the power of 2
- ReLU² creates sparsity (many zeros) in the activations which is critical for ternary quantization


### What is Sigmoid
- Sigmoid is an activation function
- Output is always between 0 and 1
- `output = sigmoid(logit) = 1 / (1 + eulers_num^(-logit))`


### What is dotproduct?
- `(weight₁ * input₁) + (weight₂ * input₂) + ...  + (weightₙ * inputₙ)`
- Multiply corresponding elements of two vectors, sum them, one number response
- Dot product is a measure of directional similarity
- How much u points in the direction of v
- A dot product:
    - > 0 tells us that 2 vectors point roughly in the same direction
    - = 0 tells us that 2 vectors are perpendicular
    - < 0 tells us that 2 vectors point in opposite direction


### What is sparsity?
- Sparsity means most values w/in a vector are zero
- Dense vector = 0% sparsity = `[1,2,3]`
- Sparse vector = 50% sparsity = `[0,1,0,2]`
- Very sparse vector = 75% sparsity = `[0,0,0,1]`
- When a vector is sparse we can:
    - Store only the non-zero values (less memory)
    - Compute only where values are non-zero (less work)


### What is ⊙?
- ⊙ is circle w/ dot or element wise multiplication
- Multiply corresponding elements of two vectors, vector response, example:
    ```txt
    a = [2, 4, 6]
    b = [1, 3, 5]

    a ⊙ b = [2×1, 4×3, 6×5] = [2, 12, 30]
    ```


### Why is Poppins written in Rust?
- **Predictable Performance:** Languages w/ a Garbage Collector (Python/Java/JavaScript) will pause programs for garbase collector maintenance. Rust does not have a garbage collector, so token generation during inference remains smooth
- **Deploy Everywhere:** Rust can deploy anywhere with 3 millisecond startup times, detect the current runtime and optimize for it & remains a tiny executable in every environment (python requires shipping the python runtime). Rust can:
    - Compile to native iOS/Android libraries
    - Compile to WASM, so Poppins models may run in the browser optimally
    - Deploy to small devices like a Rasberry Pi b/c no operating system is required 
- **Concurrency:** Python's Global Interpreter Lock (GIL) prevents true parallelism. Rust can use all CPU cores efficiently which helps us scale optimally
- **Peace:** C++ solutions like `llama.cpp` (typically called from Python via `llama-cpp-python`) can crash with memory errors that are hard to debug. With Rust developers never see common Python errors like segmentation faults, memory corruption or hard to debug crashes in production b/c Rust guarantees safety at the language level
