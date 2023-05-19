use poise::serenity_prelude as serenity;
use serde_json::Value;
use tracing::error;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Get the price of a crypto token
#[poise::command(slash_command, prefix_command)]
async fn price(
    ctx: Context<'_>,
    #[description = "(Coingecko) ID of the token"] coingecko_id: String,
) -> Result<(), Error> {
    let coingecko_url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd%2Cidr",
        coingecko_id
    );
    let res = reqwest::get(coingecko_url)
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();
    if res.get(&coingecko_id).is_some() {
        let idr = &res[&coingecko_id]["idr"];
        let usd = &res[&coingecko_id]["usd"];
        let msg = format!(":information_source: {coingecko_id} price: ${usd} / Rp. {idr}");
        ctx.say(msg).await?;
    } else {
        let msg = format!(":sob: {coingecko_id} is not valid coin");
        ctx.say(msg).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let framework: poise::FrameworkBuilder<Data, Box<dyn Error + Send + Sync>> = poise::Framework::builder()
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
                Ok(Data {})
            })
        });

    framework.run_autosharded().await.unwrap();
}
