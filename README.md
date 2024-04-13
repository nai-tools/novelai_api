# NovelAI API
Rust API client for NovelAI API

## Overview

This API client provides you access to the NovelAI API, the following endpoints have been implemented:
- [X] /ai/generate
- [X] /ai/generate-voice
- [ ] /ai/generate-image
- [ ] /user/objects

- Current Novel AI API version: 1.0

## Installation
Add the following to `Cargo.toml` under `[dependencies]`:

```
novelai_api = "0.2.0"
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

### Customizing Ai Parameters
TODO: Create Example

### Generating TTS
TODO: Create Example

### Generating Images
TODO: Create Example
