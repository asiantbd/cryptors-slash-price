# Blazingly Fast Crypto Price Discord Bot with Slash Command in Rust
---

This version is the Rust version of the C++ version [here](https://github.com/asiantbd/crypto-prices-slash-bot-cpp).


## Running the Bot
```shell
$ docker run -d -e "DISCORD_TOKEN=token" ghcr.io/asiantbd/cryptors-price-slash-bot:v0.0.1
```

--- 
Available options:

| Command | Description |
|-------|--------|
|/price | Get crypto price from coingecko|


## Features

- Support User State
- Support Graceful Shutdown when Bot Receives the Signal Interrupt
