use bytes::Bytes;
use regex::Regex;
use reqwest::RequestBuilder;
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

use crate::model::{AiGenerateRequest, Configuration};

pub fn setup_request(mut request: RequestBuilder, configuration: &Configuration) -> RequestBuilder {
    if let Some(token) = &configuration.bearer_access_token {
        request = request.bearer_auth(token);
    }
    if let Some(user_agent) = &configuration.user_agent {
        request = request.header(reqwest::header::USER_AGENT, user_agent);
    }
    request
}

#[derive(Debug, Error)]
pub enum NAIRequestError {
    #[error("Request Failed")]
    ReqwestError(#[from] reqwest::Error),
}

pub async fn ai_generate_voice(
    configuration: &Configuration,
    text: &str,
    seed: &str,
    voice: f64,
    opus: bool,
    version: &str,
) -> Result<Bytes, NAIRequestError> {
    let request_url = format!(
        "/ai/generate-voice?text={}&seed={}&voice={}&opus={}&version={}",
        text, seed, voice, opus, version
    );
    let mut request = configuration
        .client
        .get(format!("{}{}", configuration.base_path, request_url));
    request = setup_request(request, configuration);
    Ok(request.send().await?.bytes().await?)
}

pub async fn ai_generate_text(
    configuration: &Configuration,
    ai_generate_request: AiGenerateRequest,
) -> Result<AiGenerateTextResponse, NAIRequestError> {
    let mut request = configuration
        .client
        .post(format!("{}{}", configuration.base_path, "/ai/generate"));
    request = setup_request(request, configuration);
    request = request.json(&ai_generate_request);
    Ok(request
        .send()
        .await?
        .json::<AiGenerateTextResponse>()
        .await?)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiGenerateTextResponse {
    pub output: String,
}

// Split using regex but keep the delimiters
fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last_end = 0;

    for match_point in r.find_iter(text) {
        if match_point.start() != last_end {
            result.push(&text[last_end..match_point.start()]);
        }
        result.push(match_point.as_str());
        last_end = match_point.end();
    }

    if last_end != text.len() {
        result.push(&text[last_end..]);
    }

    result
}

const TTS_MAX_INPUT_LENGTH: usize = 1000;
/// Split text first by
/// , | \. | \." | ," | \?  | \?" | !  | !" | \n\n | \n | \.\.\.
/// Then as backup use wordwrap
pub fn process_string_for_voice_generation(string: impl AsRef<str>) -> Vec<String> {
    // Max length is 1000 so return if under that
    if string.as_ref().len() <= TTS_MAX_INPUT_LENGTH {
        return vec![string.as_ref().to_owned()];
    }

    let input_str = string.as_ref().to_owned();
    let natural_speach_stop_spliter =
        regex::Regex::new(r#",|\. |\."|,"|\? |\?"|! |!"|\n\n|\n|\.\.\.|…"#).unwrap();

    let natural_split_text: Vec<&str> = split_keep(&natural_speach_stop_spliter, &input_str);
    let mut split_text: Vec<String> = vec![];

    for element in natural_split_text {
        if element.len() < TTS_MAX_INPUT_LENGTH {
            split_text.push(element.to_owned());
        } else {
            let wrapped_element = textwrap::wrap(element, TTS_MAX_INPUT_LENGTH);
            split_text.append(&mut wrapped_element.iter().map(|x| x.to_string()).collect());
        }
    }

    let mut reduced_split_text = vec![split_text[0].to_owned()];

    for split_element in split_text.iter().take(split_text.len() - 1).skip(1) {
        let last_idx = reduced_split_text.len() - 1;
        if reduced_split_text[last_idx].len() + split_element.len() < TTS_MAX_INPUT_LENGTH {
            reduced_split_text[last_idx].push_str(split_element);
        } else {
            reduced_split_text.push(split_element.to_owned());
        }
    }

    reduced_split_text
}
