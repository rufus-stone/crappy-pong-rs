use rand::{Rng, RngCore};

use crate::chromosome::Chromosome;

pub trait CrossoverMethod {
    fn crossover(
        &self,
        prng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}

#[derive(Clone, Debug)]
pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        prng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());

        let mut child = Vec::new();
        let gene_count = parent_a.len();

        for gene_idx in 0..gene_count {
            let gene = if prng.gen_bool(0.5) {
                parent_a[gene_idx]
            } else {
                parent_b[gene_idx]
            };

            child.push(gene);
        }

        child.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn uniform_crossover() {
        // Seed a ChaCha8Rng for a predictable "random" number to use for testing
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        // Create two parent Chromosomes with 100 genes each
        let parent_a: Chromosome = (1..=100).map(|n| n as f32).collect();
        let parent_b: Chromosome = (1..=100).map(|n| -n as f32).collect();

        // Produce a child Chromosome using the UniformCrossover method
        let child = UniformCrossover::new().crossover(&mut prng, &parent_a, &parent_b);

        // Calculate the number of genes that differ between child and parent_a
        let diff_a = child
            .iter()
            .zip(parent_a)
            .filter(|(child_gene, parent_gene)| *child_gene != parent_gene)
            .count();

        // Calculate the number of genes that differ between child and parent_b
        let diff_b = child
            .iter()
            .zip(parent_b)
            .filter(|(child_gene, parent_gene)| *child_gene != parent_gene)
            .count();

        // Roughly looks like 50%, which proves that chance for picking either gene *is* 50%
        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
    }
}
