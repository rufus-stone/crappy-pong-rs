use ggez::*;

mod core;
mod settings;

fn main() -> GameResult {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .init()
        .unwrap();

    log::info!("Hello, World!");

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

    // Create an GameState object
    let game_state = core::GameState::new().unwrap();

    // Start the game!
    ggez::event::run(ctx, event_loop, game_state);
}
