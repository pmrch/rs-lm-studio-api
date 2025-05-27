[![github]](https://github.com/fuderis/rs-lm-studio-api)&ensp;
[![crates-io]](https://crates.io/crates/lm-studio-api)&ensp;
[![docs-rs]](https://docs.rs/lm-studio-api)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# LM Studio API

## Introduction

This API is designed for interacting with LM Studio. It allows you to send requests to locally running models, receive results, and manage model parameters. The API uses JSON for data exchange.


## Examples:

#### No Stream Using:

```rust
// init chat:
let mut chat = Chat::new(
    Model::Gemma3_4b,   // select AI model
    Context::new("You're Jarvis - my personal assistant. Call me master", 4090),  // write system prompt + set max size of messages context
    "9090"    // server IP port
);

// init request:
let request = Request {
    messages: vec!["Hi, what's your name?".into()],
    context: true,
    stream: false,  // disable stream mode
    ..Request::default()
};

// sending request:
let result = chat.send(request).await;

match result {
    // print results part:
    Ok(Some(response)) => println!("{}", response.text()),

    // print error:
    Err(e) => eprintln!("Error: {e}"),

    _ => {}
}

```

#### Stream Using (real-time generation):

```rust
use lm_studio_api::{ Model, Context, Chat, Request };

// init chat:
let mut chat = Chat::new(
    Model::Gemma3_4b,   // select AI model
    Context::new("You're Jarvis - my personal assistant. Call me master", 4090),  // write system prompt + set max size of messages context
    "9090"    // server IP port
);

loop {
    // reading user input:
    eprint!("\n>> ");
    
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    // generating answer:
    eprint!("<< ");
    
    // init request:
    let request = Request {
        messages: vec![buf.into()],
        context: true,
        stream: true,  // enable stream mode
        ..Request::default()
    };
    
    // sending request:
    let _ = chat.send(request).await.unwrap();

    // reading AI results:
    while let Some(result) = chat.next().await {
        match result {
            // print results part:
            Ok(text) if !text.is_empty() => {
                eprint!("{text}");
            },

            // print error:
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            },

            _ => {}
        }
    }
}
```

## Licensing:

Distributed under the MIT license.


## Feedback:

You can contact me via GitHub or send a message to my Telegram [@fuderis](https://t.me/fuderis).

This library is constantly evolving, and I welcome your suggestions and feedback.
