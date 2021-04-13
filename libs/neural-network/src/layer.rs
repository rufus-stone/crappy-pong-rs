use crate::neuron::Neuron;

#[derive(Debug, Clone)]
pub struct Layer {
    pub(crate) neurons: Vec<Neuron>,
}

impl Layer {
    /// Create a new Layer with the specified neurons
    pub fn new(neurons: Vec<Neuron>) -> Self {
        assert!(!neurons.is_empty());

        Self { neurons }
    }

    /// Create a new Layer with randomly chosen neurons
    pub fn random(
        prng: &mut dyn rand::RngCore,
        input_neurons: usize,
        output_neurons: usize,
    ) -> Layer {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(prng, input_neurons))
            .collect();

        Layer { neurons }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // This can be re-written using .map()
        // Using the .iter() method also implicitely calls Vec::with_capacity() which is nice
        //let mut outputs = Vec::with_capacity(self.neurons.len());
        //for neuron in &self.neurons {
        //    let output = neuron.propagate(&inputs);
        //    outputs.push(output);
        //}
        //outputs

        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random_layer_creation() {
        // Seed a ChaCha8Rng for a predictable "random" number to use for testing
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        // Roll a new random Layer with 3 input Neurons and 2 output Neurons
        let layer = Layer::random(&mut prng, 3, 2);

        // Collect together the biases of each neuron in the layer
        let actual_biases: Vec<f32> = layer.neurons.iter().map(|neuron| neuron.bias).collect();

        // Given the default random seed, these should be the biases of the neurons in our layer
        let expected_biases = vec![-0.6255188, 0.5238807];

        // Check the actual biases match the expected biases
        approx::assert_relative_eq!(actual_biases.as_slice(), expected_biases.as_slice());

        // Collect together all the weights of each neuron in the layer
        let actual_weights: Vec<&[f32]> = layer
            .neurons
            .iter()
            .map(|neuron| neuron.weights.as_slice())
            .collect();

        // Given the default random seed, these should be the weights of each neuron in our layer
        let expected_weights: Vec<&[f32]> = vec![
            &[0.67383957, 0.8181262, 0.26284897],
            &[-0.53516835, 0.069369674, -0.7648182],
        ];

        // Check the actual weights match the expected weights
        approx::assert_relative_eq!(actual_weights.as_slice(), expected_weights.as_slice());
    }
}
