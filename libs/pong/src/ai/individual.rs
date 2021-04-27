use genetic_algorithm as ga;

use ga::chromosome::*;
use ga::individual::Individual;

use super::player::*;

#[derive(Debug)]
pub struct AiIndividual {
    pub chromosome: Chromosome,
    pub fitness: f32,
}

impl AiIndividual {
    pub fn new(ai_player: &AiPlayer) -> Self {
        log::warn!("Creating new AiIndividual from an AiPlayer...");
        Self {
            chromosome: ai_player.brain.to_chromosome(),
            fitness: ai_player.score as f32,
        }
    }
}

impl Individual for AiIndividual {
    fn create(chromosome: Chromosome) -> Self {
        log::warn!("Creating new AiIndividual from a Chromosome...");
        Self {
            chromosome,
            fitness: 0.0,
        }
    }

    fn chromosome(&self) -> &Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}
