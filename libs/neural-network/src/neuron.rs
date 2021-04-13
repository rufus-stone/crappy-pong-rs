use rand::Rng;

#[derive(Debug, Clone)]
pub struct Neuron {
    pub(crate) bias: f32,
    pub(crate) weights: Vec<f32>,
}

impl Neuron {
    /// Create a new Neuron with the specified bias and weights
    pub fn new(bias: f32, weights: Vec<f32>) -> Self {
        assert!(!weights.is_empty());

        Self { bias, weights }
    }

    /// Create a new Neuron with randomly chosen bias and weights
    pub fn random(prng: &mut dyn rand::RngCore, output_size: usize) -> Neuron {
        // Create a PRNG
        //let mut prng = rand::thread_rng();

        // Generate a random bias (between -1 and 1 inclusive)
        let bias = prng.gen_range(-1.0..=1.0);

        // Generate random weights (between -1 and 1 inclusive)
        let weights = (0..output_size)
            .map(|_| prng.gen_range(-1.0..=1.0))
            .collect();

        Neuron { bias, weights }
    }

    /// Combine the inputs and propogate the output
    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        // There should always be an equal number of inputs and weights (as the weights modify each input)
        assert_eq!(inputs.len(), self.weights.len());

        //let mut output = 0.0;

        // This can be re-written using .zip()
        //for i in 0..inputs.len() {
        //    output += inputs[i] * self.weights[i];
        //}

        // This can also be re-written using a .map()
        //for (&input, &weight) in inputs.iter().zip(&self.weights) {
        //    output += input * weight;
        //}

        // Take each input, multiply it by the corresponding weight, and sum all the results together
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        //output += self.bias;
        //output.max(0.0) // Return the whatever is the largest of output and 0.0

        // Finally, add the bias to the sum, and return whichever is the bigger of that value or 0.0
        (self.bias + output).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random_neuron_creation() {
        // Seed a ChaCha8Rng for a predictable "random" number to use for testing
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        // Roll a new random Neuron with 4 outputs
        let neuron = Neuron::random(&mut prng, 4);

        // Check the bias of the neuron
        approx::assert_relative_eq!(neuron.bias, -0.6255188);

        // Check the number of weights for the neuron
        assert_eq!(neuron.weights.len(), 4);

        // Check the weights of the neuron
        let expected_weights = vec![0.67383957, 0.8181262, 0.26284897, 0.5238807];
        approx::assert_relative_eq!(neuron.weights.as_slice(), expected_weights.as_slice());
    }

    #[test]
    fn neuron_propogation() {
        // Create a new Neuron with the specified bias and weights
        let neuron = Neuron::new(0.1, vec![-0.3, 0.6, 0.9]);

        // Make up some fake input data and calculate the propogated value
        let propogated = neuron.propagate(&[0.5, -0.6, 0.7]);

        // This is effectively the calculation the .propogate() function should be performing
        let expected = ((0.1 + (0.5 * -0.3) + (-0.6 * 0.6) + (0.7 * 0.9)) as f32).max(0.0);

        // Check the results of the propogation match what we expected
        approx::assert_relative_eq!(propogated, expected);
    }

    #[test]
    fn neuron_output_restriction() {
        // Create a new Neuron with the specified bias and weights
        let neuron = Neuron::new(0.0, vec![0.5]);

        // Calculate the propogated value given various different input values
        let v1 = neuron.propagate(&[-1.0]); // 0.0 + (-1.0 * 0.5) == -0.5 so this should produce 0.0
        let v2 = neuron.propagate(&[-0.5]); // 0.0 + (-0.5 * 0.5) == -0.25 so this should produce 0.0
        let v3 = neuron.propagate(&[0.0]); // 0.0 + (0.0 * 0.5) == 0.0 so this should produce 0.0
        let v4 = neuron.propagate(&[0.5]); // 0.0 + (0.5 * 0.5) == 0.25 so this should produce 0.25
        let v5 = neuron.propagate(&[1.0]); // 0.0 + (1.0 * 0.5) == 0.5 so this should produce 0.5

        approx::assert_relative_eq!(v1, 0.0);
        approx::assert_relative_eq!(v2, 0.0);
        approx::assert_relative_eq!(v3, 0.0);
        approx::assert_relative_eq!(v4, 0.25);
        approx::assert_relative_eq!(v5, 0.5);

        // Create another new Neuron with a different bias and weights
        let neuron = Neuron::new(2.0, vec![0.15]);

        // Calculate the propogated value given various different input values
        let v1 = neuron.propagate(&[-1.0]); // 2.0 + (-1.0 * 0.15) == 1.85 so this should produce 1.85
        let v2 = neuron.propagate(&[-0.5]); // 2.0 + (-0.5 * 0.15) == 1.925 so this should produce 1.925
        let v3 = neuron.propagate(&[0.0]); // 2.0 + (0.0 * 0.15) == 2.0 so this should produce 2.0
        let v4 = neuron.propagate(&[0.5]); // 2.0 + (0.5 * 0.15) == 2.075 so this should produce 2.075
        let v5 = neuron.propagate(&[-7.0]); // 2.0 + (-7.0 * 0.15) == 0.95 so this should produce 0.95

        approx::assert_relative_eq!(v1, 1.85);
        approx::assert_relative_eq!(v2, 1.925);
        approx::assert_relative_eq!(v3, 2.0);
        approx::assert_relative_eq!(v4, 2.075);
        approx::assert_relative_eq!(v5, 0.95);
    }
}
