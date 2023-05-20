mod commands;
mod state;

use commands::price;
use poise::serenity_prelude as serenity;
use state::State;
use tracing::error;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let framework: poise::FrameworkBuilder<State, Box<dyn std::error::Error + Send + Sync>> =
        poise::Framework::builder()
            .options(poise::FrameworkOptions {
                commands: vec![price()],
                on_error: |error| Box::pin(async move { error!("{error}") }),
                ..Default::default()
            })
            .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
            .intents(serenity::GatewayIntents::non_privileged())
            .setup(|ctx, _ready, framework| {
                Box::pin(async move {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    let sm = framework.shard_manager().clone();
                    tokio::spawn(async move {
                        tokio::signal::ctrl_c()
                            .await
                            .expect("failed to listen for ctrl+c");
                        sm.lock().await.shutdown_all().await;
                    });
                    Ok(State {})
                })
            });

    framework.run_autosharded().await.unwrap();
}
