use crate::state::State;
use serde_json::Value;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn price(
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

    match res.get(&coingecko_id) {
        Some(value) => {
            let idr = value.get("idr").unwrap();
            let usd = value.get("usd").unwrap();
            let msg = format!(":information_source: {coingecko_id} price: ${usd} / Rp. {idr}");
            ctx.say(msg).await?;
        }
        None => {
            let msg = format!(":sob: {coingecko_id} is not valid coin");
            ctx.say(msg).await?;
        }
    }
    Ok(())
}
