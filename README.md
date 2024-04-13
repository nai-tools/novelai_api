# NovelAI API
Rust API client for NovelAI API

## Overview

This API client provides you access to the NovelAI API, the following endpoints have been implemented:
- Current Novel AI API version: 1.0

## Installation
Add the following to `Cargo.toml` under `[dependencies]`:

```
novelai_api = "0.2"
```
or by running
```bash
cargo add novelai_api
```


## Documentation
Documentation can be found at:
- [API Documentation NovelAI (https://api.novelai.net/docs)](https://api.novelai.net/docs/)
- [Crate Documentation docs.rs (https://docs.rs/novelai_api/latest/novelai_api)](https://docs.rs/novelai_api/latest/novelai_api/)

## Example Usage

### Generating Text
```rust
use novelai_api::{api::ai_generate_text, model::*};

#[tokio::main]
async fn main() {
    let mut conf = Configuration::new();
    conf.bearer_access_token =
        Some("Your Token".to_string());

    let prompt = "Tell me about the lost world!".to_string();
    let response = ai_generate_text(
        &conf,
        AiGenerateRequest::new(
            prompt.clone(),
            novelai_api::model::TextModel::KayraV1,
            AiGenerateParameters::new(),
        ),
    )
    .await.unwrap();

    println!("{}{}", prompt, response.output);
    /*
    Tell me about the lost world! I urged him. "I'm going there in two days' time."
    "What? Are you mad?" He sounded incredulous.
    "Not yet, but I am getting that way," I told him, with a grin. "The 
    expedition was a last-minute addition to my schedule. They have been trying 
    to reach a group of natives called the Chirpsithra for years. The Chirps 
    have asked me to join the party as a liaison."
    */
}
```

### Generating TTS
```rust
use novelai_api::{api::*, model::Configuration};
use std::fs;

#[tokio::main]
async fn main() {
    let prompt = "Hello world!";

    // API has a limit of 1000 Characters per request
    // due to this we split our string at the
    // nearest vocal pause close to 1000 chars
    let prompt: Vec<String> = process_string_for_voice_generation(prompt);

    let mut conf = Configuration::new();
    conf.bearer_access_token = Some("Your Token".to_string());

    for (i, chunk) in prompt.iter().enumerate() {
        let output = ai_generate_voice(&conf, chunk, "Aini", -1.0, true, "v2")
            .await
            .unwrap();

        // output now is string of .webm audio file
        // which you could save to a file
        fs::write(format!("./{}_output.webm", i), output).unwrap();
    }
}
```


### Customizing Ai Parameters
TODO: Create Example
```rust
use novelai_api::{api::ai_generate_text, model::*};

#[tokio::main]
async fn main() {
    let model = novelai_api::model::TextModel::KayraV1;
    /* Model Options
        Variant2Period7B,
        Variant6Bv4,
        EuterpeV2,
        GenjiPython6b,
        GenjiJp6b,
        GenjiJp6bV2,
        KrakeV2,
        Hypebot,
        Infillmodel,
        Cassandra,
        Sigurd2Period9bV1,
        Blue,
        Red,
        Green,
        Purple,
        ClioV1,
        KayraV1,
    */

    let generation_parameters = AiGenerateParameters {
        temperature: Some(2.8),
        min_length: 50,
        max_length: 300,
        top_a: Some(1.0),
        // (A bunch more settings Avalible)
        ..Default::default()
    };

    let request_settings = AiGenerateRequest::new(
        "Your Prompt".to_string(),
        model,
        generation_parameters
    );

    let mut conf = Configuration::new();
    conf.bearer_access_token =
        Some("Your Token".to_string());

    let output = ai_generate_text(&conf, request_settings).await.unwrap();
    println!("{}", output.output);
}
```

