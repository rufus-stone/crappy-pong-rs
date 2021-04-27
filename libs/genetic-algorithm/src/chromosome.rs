use std::{iter::FromIterator, ops::Index};

#[derive(Clone, Debug, PartialEq)]
pub struct Chromosome {
    genes: Vec<f32>,
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.genes.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }

    pub fn as_slice(&self) -> &[f32] {
        self.genes.as_slice()
    }
}

// The Index trait allows indexing into a custom type using the [i] syntax
impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

// The FromIterator trait allows using .collect() into your a custom type
impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

// The IntoIterator trait turns a custom type into an iterator
impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_test() {
        let chromosome = Chromosome {
            genes: vec![3.0, 1.0, 2.0],
        };

        assert_eq!(chromosome.len(), 3);
    }

    #[test]
    fn chromosome_iter() {
        let chromosome = Chromosome {
            genes: vec![3.0, 1.0, 2.0],
        };

        // Collect the genes of the chromosome into a new Vec<&f32>
        let actual: Vec<&f32> = chromosome.iter().collect();

        let expected: Vec<&f32> = vec![&3.0, &1.0, &2.0];

        assert_eq!(actual, expected);

        assert_eq!(actual.len(), 3);
        approx::assert_relative_eq!(actual[0], &3.0);
        approx::assert_relative_eq!(actual[1], &1.0);
        approx::assert_relative_eq!(actual[2], &2.0);
    }

    #[test]
    fn chromosome_into_iter() {
        let chromosome = Chromosome {
            genes: vec![3.0, 1.0, 2.0],
        };

        // Collect the genes of the chromosome into a new Vec<&f32>
        let actual: Vec<f32> = chromosome.into_iter().collect();

        let expected: Vec<f32> = vec![3.0, 1.0, 2.0];

        assert_eq!(actual, expected);

        assert_eq!(actual.len(), 3);
        approx::assert_relative_eq!(actual[0], 3.0);
        approx::assert_relative_eq!(actual[1], 1.0);
        approx::assert_relative_eq!(actual[2], 2.0);
    }

    #[test]
    fn chromosome_to_chromosome() {
        let chromosome = Chromosome {
            genes: vec![3.0, 1.0, 2.0],
        };

        let chromosome_copy = chromosome.clone();

        // Collect the genes of the chromosome into a new Vec<f32> (this consumes the chromosome)
        let new_chromosome: Chromosome = chromosome.into_iter().collect();

        assert_eq!(chromosome_copy, new_chromosome);
    }

    #[test]
    fn chromosome_from_iter() {
        let genes: Vec<f32> = vec![3.0, 1.0, 2.0];

        let chromosome: Chromosome = genes.clone().into_iter().collect();

        approx::assert_relative_eq!(chromosome[0], genes[0]);
        approx::assert_relative_eq!(chromosome[1], genes[1]);
        approx::assert_relative_eq!(chromosome[2], genes[2]);
    }
}
