use rand::RngCore;

use crate::player::Move;
use crate::player::Snapshot;
use crate::settings::*;

use super::brain::*;
use super::eye::*;

pub struct AiPlayer {
    pub(crate) brain: Brain,
    pub(crate) eye: Eye,
    pub(crate) config: Config,
    pub(crate) score: i16,
}

impl AiPlayer {
    pub fn random(config: &Config, rng: &mut dyn RngCore) -> AiPlayer {
        log::warn!("New random AI player");
        let brain = Brain::random(config, rng);

        AiPlayer {
            brain,
            eye: Eye::new(config),
            config: config.clone(),
            score: 0,
        }
    }

    fn random_move() -> f32 {
        use rand::Rng;

        let mut prng = rand::thread_rng();

        let flip = prng.gen::<bool>();

        if flip {
            PADDLE_SPEED
        } else {
            -PADDLE_SPEED
        }
    }

    pub fn step(&self, _snapshot: &Snapshot) -> f32 {
        // Break out the paddle and ball from the snapshot of game state
        let paddle = _snapshot.paddle;
        let ball = _snapshot.ball.clone();

        // First, check what we can see
        let eye = self.eye.step(&self.config, paddle, &ball);

        // Second, think about it
        self.brain.step(&self.config, &eye)
    }
}

impl Move for AiPlayer {
    fn make_move(&self, ctx: &mut ggez::Context, _snapshot: &Snapshot) -> f32 {
        let desired_move = self.step(_snapshot);

        if desired_move < 0.0 {
            -PADDLE_SPEED
        } else if desired_move > 0.0 {
            PADDLE_SPEED
        } else {
            0.0
        }
    }

    fn name(&self) -> &'static str {
        "AI"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random_ai_player() {
        // Seed a ChaCha8Rng for a predictable "random" number to use for testing
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        // Create a new default config
        let config = Config::default();

        // Create a new random AI player
        let ai_player = AiPlayer::random(&config, &mut prng);

        // A default config produces 2 layers...
        assert_eq!(ai_player.brain.network().layers().len(), 2);

        // ...with 15 neurons in the first layer
        assert_eq!(ai_player.brain.network().layers()[0].neurons().len(), 15);

        // ...and 1 neuron in the second layer
        assert_eq!(ai_player.brain.network().layers()[1].neurons().len(), 1);

        // Check the bias of the first neuron of the first layer
        approx::assert_relative_eq!(
            ai_player.brain.network().layers()[0].neurons()[0].bias(),
            -0.6255188
        );

        // Check the weights of the first neuron of the second layer
        let expected_weights = vec![
            -0.3431031,
            -0.8963325,
            0.053979516,
            -0.5960805,
            -0.8928735,
            -0.22718108,
            0.80183077,
            0.070950866,
            0.273749,
            -0.25688833,
            0.5900805,
            0.6363394,
            -0.29444236,
            0.5295007,
            0.4402212,
        ];
        approx::assert_relative_eq!(
            ai_player.brain.network().layers()[1].neurons()[0].weights(),
            expected_weights.as_slice()
        );

        // Check the number of weights for the second neuron of the first layer (this is the same as the number of inputs it takes)
        assert_eq!(
            ai_player.brain.network().layers()[0].neurons()[1]
                .weights()
                .len(),
            5
        );
    }
}
