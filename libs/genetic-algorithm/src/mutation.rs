use rand::{Rng, RngCore};

use crate::chromosome::Chromosome;

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    /// Probability of changing a gene:
    /// 0.0 = no genes will be touched
    /// 1.0 = all genes will be touched
    chance: f32,

    /// Magnitude of that change:
    /// 0.0 = touched genes will not be modified
    /// 3.0 = touched genes will be += or -= by at most 3.0
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!((0.0..=1.0).contains(&chance));

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, prng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if prng.gen_bool(0.5) { -1.0 } else { 1.0 };

            if prng.gen_bool(self.chance as _) {
                *gene += sign * self.coeff * prng.gen::<f32>();
            }
        }
    }
}

pub trait MutationMethod {
    fn mutate(&self, prng: &mut dyn RngCore, child: &mut Chromosome);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn gaussian_mutation() {
        // Seed a ChaCha8Rng for a predictable "random" number to use for testing
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        // ------------------------------------------------------------------------------------------
        // Create a mutator with a 50% chance of mutating each gene by 0 (i.e. nothing ever happens!)
        let mutator = GaussianMutation::new(0.5, 0.0);

        // Create a fake child to mutate
        let mut child: Chromosome = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

        // Given the default random seed and the given mutation settings, mutating the child should produce the following Chromosome (i.e. nothing will change)
        let expected: Chromosome = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

        // Mutate the child
        mutator.mutate(&mut prng, &mut child);

        // Check the actual mutation matches the expected mutation
        approx::assert_relative_eq!(child.as_slice(), expected.as_slice());

        // ---------------------------------------------------------------------------
        // Create a mutator with a 75% chance of mutating each gene by at most +/- 2.0
        let mutator = GaussianMutation::new(0.75, 2.0);

        // Create a fake child to mutate
        let mut child: Chromosome = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

        // Given the default random seed and the given mutation settings, mutating the child should produce the following Chromosome
        let expected: Chromosome = vec![0.32473612, 2.0, 4.998356, 5.6518774, 5.3180294]
            .into_iter()
            .collect();

        // Mutate the child
        mutator.mutate(&mut prng, &mut child);

        // Check the actual mutation matches the expected mutation
        approx::assert_relative_eq!(child.as_slice(), expected.as_slice());

        // ---------------------------------------------------------------------------
        // Create a mutator with a 100% chance of mutating each gene by at most +/- 0.25
        let mutator = GaussianMutation::new(1.0, 0.25);

        // Create a fake child to mutate
        let mut child: Chromosome = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

        // Given the default random seed and the given mutation settings, mutating the child should produce the following Chromosome
        let expected: Chromosome = vec![1.2004075, 2.0767758, 3.003996, 4.0161314, 4.8749585]
            .into_iter()
            .collect();

        // Mutate the child
        mutator.mutate(&mut prng, &mut child);

        // Check the actual mutation matches the expected mutation
        approx::assert_relative_eq!(child.as_slice(), expected.as_slice());
    }
}
