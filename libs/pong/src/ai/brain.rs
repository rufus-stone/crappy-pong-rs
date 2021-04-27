use rand::RngCore;

use genetic_algorithm as ga;
use neural_network as nn;

use ga::chromosome::*;

use super::eye::*;
use crate::settings::*;

#[derive(Clone, Debug)]
pub struct Brain {
    network: nn::Network,
}

impl Brain {
    /// Create a new brain with a random Neural Network
    pub fn random(config: &Config, rng: &mut dyn RngCore) -> Brain {
        let network = nn::Network::random(rng, &Self::network_topology(config));

        Brain { network }
    }

    /// Generate a neural network LayerTopology given the provided Config
    fn network_topology(config: &Config) -> [nn::topology::LayerTopology; 3] {
        [
            nn::topology::LayerTopology {
                neurons: config.eye_photoreceptors,
            },
            nn::topology::LayerTopology {
                neurons: config.brain_neurons,
            },
            nn::topology::LayerTopology {
                neurons: config.outputs,
            },
        ]
    }

    pub fn to_chromosome(&self) -> Chromosome {
        self.network.weights().collect()
    }

    /// Get immutable borrow of the brain
    pub fn network(&self) -> &nn::Network {
        &self.network
    }

    pub fn step(&self, config: &Config, eye: &Eye) -> f32 {
        let response = self.network.propagate(eye.energies.clone());
        log::warn!("Brain prop: {:?}", &response);
        response[0]
    }
}
