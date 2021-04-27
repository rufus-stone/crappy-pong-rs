use ggez::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use pong::settings::*;
use pong::*;

fn main() -> GameResult {
    // Turn on logging
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .init()
        .unwrap();

    // What kind of game are we playing? 2 player, 1 player, etc.?
    let game_mode = cli::get_game_mode().unwrap_or(PLAYER_VS_PLAYER);

    log::warn!("game_mode: {:?}", &game_mode);

    // Create a new ggez Context and EventsLoop
    let (ctx, event_loop) = ContextBuilder::new(settings::GAME_TITLE, "Rufus Stone")
        .window_setup(
            conf::WindowSetup::default()
                .title(settings::GAME_TITLE)
                .vsync(true),
        )
        .window_mode(
            conf::WindowMode::default().dimensions(settings::SCREEN_WIDTH, settings::SCREEN_HEIGHT),
        )
        .build()
        .unwrap();

    // Create a GameState object
    let mut prng = ChaCha8Rng::from_seed(Default::default());
    let game_state = core::GameState::new(game_mode, &mut prng).unwrap();

    // Start the game!
    ggez::event::run(ctx, event_loop, game_state);
}
