## Absolute Value
- The absolute value of a number is its distance from zero, ignoring whether it's positive or negative


## avg_abs
- the average absolute value of all weights
- calculation
    - γ = (self.raw.iter().map(|&w| w.abs()).sum()) / self.size
- helpful anytime we quantize weights
    - let normalized = weight / gamma
    - helps us normalize weights before quantization (choosing if their -1, 0 or 1)


## step_counter
- each time we update weights we increment the step_counter
- how many steps do we weight before reupdating gamma


## Input
- An input is a value coming into the layer from the original data or the output of a previous layer
- Could be vocab_size or hidden_dim


## Weight
- A weight is a single number that gets multiplied by an input
- Raw weights are `f32`
- Quantized weights are `-1`, `0` or `1`


## Output
- Provided by a layer after altering inputs based on weights and bias 
- Could be embedding_dim or hidden_dim


## Neuron
- Has 1 weight for each input
- Produces an output value
- Identified by it's `out_idx`


## Layer
- Collection of neurons
- Has an input_size: count of numbers it processes
