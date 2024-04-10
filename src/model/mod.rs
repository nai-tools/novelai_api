use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub bearer_access_token: Option<String>,
}

impl Configuration {
    pub fn new() -> Self {
        Configuration::default()
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "https://api.novelai.net".to_owned(),
            user_agent: Some(format!("NovelAI-API-Client/{}/rust", VERSION)),
            client: reqwest::Client::new(),
            bearer_access_token: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AiGenerateParametersLogitBiasExpInner {
    #[serde(rename = "sequence")]
    pub sequence: Vec<i32>,
    #[serde(rename = "bias")]
    pub bias: f32,
    #[serde(
        rename = "ensure_sequence_finish",
        skip_serializing_if = "Option::is_none"
    )]
    pub ensure_sequence_finish: Option<bool>,
    #[serde(rename = "generate_once", skip_serializing_if = "Option::is_none")]
    pub generate_once: Option<bool>,
}

impl AiGenerateParametersLogitBiasExpInner {
    pub fn new(sequence: Vec<i32>, bias: f32) -> AiGenerateParametersLogitBiasExpInner {
        AiGenerateParametersLogitBiasExpInner {
            sequence,
            bias,
            ensure_sequence_finish: None,
            generate_once: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AiGenerateParameters {
    #[serde(rename = "stop_sequences", skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<Vec<i32>>>,
    #[serde(rename = "bad_words_ids", skip_serializing_if = "Option::is_none")]
    pub bad_words_ids: Option<Vec<Vec<i32>>>,
    /// If false, input and output strings should be Base64-encoded uint16 numbers representing tokens
    #[serde(rename = "use_string", skip_serializing_if = "Option::is_none")]
    pub use_string: Option<bool>,
    #[serde(rename = "logit_bias", skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<Vec<Vec<i32>>>,
    #[serde(rename = "logit_bias_exp", skip_serializing_if = "Option::is_none")]
    pub logit_bias_exp: Option<Vec<AiGenerateParametersLogitBiasExpInner>>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<Vec<i32>>,
    #[serde(
        rename = "repetition_penalty_whitelist",
        skip_serializing_if = "Option::is_none"
    )]
    pub repetition_penalty_whitelist: Option<Vec<i32>>,
    #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(rename = "min_length")]
    pub min_length: u32,
    #[serde(rename = "max_length")]
    pub max_length: u32,
    #[serde(rename = "do_sample", skip_serializing_if = "Option::is_none")]
    pub do_sample: Option<bool>,
    #[serde(rename = "early_stopping", skip_serializing_if = "Option::is_none")]
    pub early_stopping: Option<bool>,
    #[serde(rename = "num_beams", skip_serializing_if = "Option::is_none")]
    pub num_beams: Option<f64>,
    #[serde(rename = "top_k", skip_serializing_if = "Option::is_none")]
    pub top_k: Option<f64>,
    #[serde(rename = "top_a", skip_serializing_if = "Option::is_none")]
    pub top_a: Option<f64>,
    #[serde(rename = "top_p", skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(rename = "typical_p", skip_serializing_if = "Option::is_none")]
    pub typical_p: Option<f64>,
    #[serde(rename = "repetition_penalty", skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f64>,
    #[serde(rename = "pad_token_id", skip_serializing_if = "Option::is_none")]
    pub pad_token_id: Option<f64>,
    #[serde(rename = "bos_token_id", skip_serializing_if = "Option::is_none")]
    pub bos_token_id: Option<f64>,
    #[serde(rename = "eos_token_id", skip_serializing_if = "Option::is_none")]
    pub eos_token_id: Option<f64>,
    #[serde(rename = "length_penalty", skip_serializing_if = "Option::is_none")]
    pub length_penalty: Option<f64>,
    #[serde(
        rename = "no_repeat_ngram_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub no_repeat_ngram_size: Option<f64>,
    #[serde(
        rename = "encoder_no_repeat_ngram_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub encoder_no_repeat_ngram_size: Option<f64>,
    #[serde(
        rename = "num_return_sequences",
        skip_serializing_if = "Option::is_none"
    )]
    pub num_return_sequences: Option<f64>,
    #[serde(rename = "max_time", skip_serializing_if = "Option::is_none")]
    pub max_time: Option<f64>,
    #[serde(rename = "use_cache", skip_serializing_if = "Option::is_none")]
    pub use_cache: Option<bool>,
    #[serde(rename = "num_beam_groups", skip_serializing_if = "Option::is_none")]
    pub num_beam_groups: Option<f64>,
    #[serde(rename = "diversity_penalty", skip_serializing_if = "Option::is_none")]
    pub diversity_penalty: Option<f64>,
    #[serde(rename = "tail_free_sampling", skip_serializing_if = "Option::is_none")]
    pub tail_free_sampling: Option<f64>,
    #[serde(
        rename = "repetition_penalty_range",
        skip_serializing_if = "Option::is_none"
    )]
    pub repetition_penalty_range: Option<f64>,
    #[serde(
        rename = "repetition_penalty_slope",
        skip_serializing_if = "Option::is_none"
    )]
    pub repetition_penalty_slope: Option<f64>,
    #[serde(rename = "get_hidden_states", skip_serializing_if = "Option::is_none")]
    pub get_hidden_states: Option<bool>,
    #[serde(
        rename = "repetition_penalty_frequency",
        skip_serializing_if = "Option::is_none"
    )]
    pub repetition_penalty_frequency: Option<f64>,
    #[serde(
        rename = "repetition_penalty_presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub repetition_penalty_presence: Option<f64>,
    #[serde(rename = "next_word", skip_serializing_if = "Option::is_none")]
    pub next_word: Option<bool>,
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(
        rename = "output_nonzero_probs",
        skip_serializing_if = "Option::is_none"
    )]
    pub output_nonzero_probs: Option<bool>,
    #[serde(
        rename = "generate_until_sentence",
        skip_serializing_if = "Option::is_none"
    )]
    pub generate_until_sentence: Option<bool>,
    #[serde(rename = "num_logprobs", skip_serializing_if = "Option::is_none")]
    pub num_logprobs: Option<f64>,
    #[serde(rename = "cfg_uc", skip_serializing_if = "Option::is_none")]
    pub cfg_uc: Option<String>,
    #[serde(rename = "cfg_scale", skip_serializing_if = "Option::is_none")]
    pub cfg_scale: Option<f64>,
    #[serde(rename = "cfg_alpha", skip_serializing_if = "Option::is_none")]
    pub cfg_alpha: Option<f64>,
    #[serde(rename = "phrase_rep_pen", skip_serializing_if = "Option::is_none")]
    pub phrase_rep_pen: Option<String>,
    #[serde(rename = "top_g", skip_serializing_if = "Option::is_none")]
    pub top_g: Option<f64>,
    #[serde(rename = "mirostat_tau", skip_serializing_if = "Option::is_none")]
    pub mirostat_tau: Option<f64>,
    #[serde(rename = "mirostat_lr", skip_serializing_if = "Option::is_none")]
    pub mirostat_lr: Option<f64>,
}

lazy_static! {
    static ref CAREFREE_REPETION_PENALITY_WHITELIST: Vec<i32> = vec![
        49256, 49264, 49231, 49230, 49287, 85, 49255, 49399, 49262, 336, 333, 432, 363, 468, 492,
        745, 401, 426, 623, 794, 1096, 2919, 2072, 7379, 1259, 2110, 620, 526, 487, 16562, 603,
        805, 761, 2681, 942, 8917, 653, 3513, 506, 5301, 562, 5010, 614, 10942, 539, 2976, 462,
        5189, 567, 2032, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 588, 803, 1040, 49209,
        4, 5, 6, 7, 8, 9, 10, 11, 12
    ];
    static ref CAREFREE_BAD_WORD_LIST: Vec<Vec<i32>> = [
        vec![3],
        vec![49356],
        vec![1431],
        vec![31715],
        vec![34387],
        vec![20765],
        vec![30702],
        vec![10691],
        vec![49333],
        vec![1266],
        vec![19438],
        vec![43145],
        vec![26523],
        vec![41471],
        vec![2936],
        vec![85, 85],
        vec![49332],
        vec![7286],
        vec![1115]
    ]
    .to_vec();
}

impl Default for AiGenerateParameters {
    fn default() -> Self {
        Self {
            stop_sequences: None,
            bad_words_ids: Some(CAREFREE_BAD_WORD_LIST.clone()),
            use_string: Some(true),
            logit_bias: None,
            logit_bias_exp: Some(vec![
                AiGenerateParametersLogitBiasExpInner {
                    bias: -0.08,
                    ensure_sequence_finish: Some(false),
                    generate_once: Some(false),
                    sequence: vec![23],
                },
                AiGenerateParametersLogitBiasExpInner {
                    bias: -0.08,
                    ensure_sequence_finish: Some(false),
                    generate_once: Some(false),
                    sequence: vec![21],
                },
            ]),
            order: Some(vec![2, 3, 0, 4, 1]),
            repetition_penalty_whitelist: Some(CAREFREE_REPETION_PENALITY_WHITELIST.clone()),
            temperature: Some(1.35),
            min_length: 1,
            max_length: 2048,
            do_sample: None,
            early_stopping: None,
            num_beams: None,
            top_k: Some(15.0),
            top_a: Some(0.1),
            top_p: Some(0.85),
            typical_p: Some(1.0),
            repetition_penalty: Some(2.8),
            pad_token_id: None,
            bos_token_id: None,
            eos_token_id: None,
            length_penalty: None,
            no_repeat_ngram_size: None,
            encoder_no_repeat_ngram_size: None,
            num_return_sequences: None,
            max_time: None,
            use_cache: None,
            num_beam_groups: None,
            diversity_penalty: None,
            tail_free_sampling: Some(0.915),
            repetition_penalty_range: Some(2048.0),
            repetition_penalty_slope: Some(0.02),
            get_hidden_states: None,
            repetition_penalty_frequency: Some(0.02),
            repetition_penalty_presence: Some(0.0),
            next_word: None,
            prefix: None,
            output_nonzero_probs: None,
            generate_until_sentence: None,
            num_logprobs: None,
            cfg_uc: None,
            cfg_scale: Some(1.0),
            cfg_alpha: None,
            phrase_rep_pen: None,
            top_g: Some(0.0),
            mirostat_tau: Some(0.0),
            mirostat_lr: Some(1.0),
        }
    }
}

impl AiGenerateParameters {
    pub fn new() -> AiGenerateParameters {
        AiGenerateParameters::default()
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AiGenerateRequest {
    /// Input for the text generation model
    #[serde(rename = "input")]
    pub input: String,
    /// Used text generation model
    #[serde(rename = "model")]
    pub model: TextModel,
    /// Generation parameters
    #[serde(rename = "parameters")]
    pub parameters: AiGenerateParameters,
}

impl AiGenerateRequest {
    pub fn new(
        input: String,
        model: TextModel,
        parameters: AiGenerateParameters,
    ) -> AiGenerateRequest {
        AiGenerateRequest {
            input,
            model,
            parameters,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TextModel {
    #[serde(rename = "2.7B")]
    Variant2Period7B,
    #[serde(rename = "6B-v4")]
    Variant6Bv4,
    #[serde(rename = "euterpe-v2")]
    EuterpeV2,
    #[serde(rename = "genji-python-6b")]
    GenjiPython6b,
    #[serde(rename = "genji-jp-6b")]
    GenjiJp6b,
    #[serde(rename = "genji-jp-6b-v2")]
    GenjiJp6bV2,
    #[serde(rename = "krake-v2")]
    KrakeV2,
    #[serde(rename = "hypebot")]
    Hypebot,
    #[serde(rename = "infillmodel")]
    Infillmodel,
    #[serde(rename = "cassandra")]
    Cassandra,
    #[serde(rename = "sigurd-2.9b-v1")]
    Sigurd2Period9bV1,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "purple")]
    Purple,
    #[serde(rename = "clio-v1")]
    ClioV1,
    #[serde(rename = "kayra-v1")]
    KayraV1,
}

impl Default for TextModel {
    fn default() -> Self {
        Self::KayraV1
    }
}

/// Used image generation model
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ImageModel {
    #[serde(rename = "nai-diffusion")]
    NaiDiffusion,
    #[serde(rename = "safe-diffusion")]
    SafeDiffusion,
    #[serde(rename = "nai-diffusion-furry")]
    NaiDiffusionFurry,
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "nai-diffusion-inpainting")]
    NaiDiffusionInpainting,
    #[serde(rename = "nai-diffusion-3-inpainting")]
    NaiDiffusion3Inpainting,
    #[serde(rename = "safe-diffusion-inpainting")]
    SafeDiffusionInpainting,
    #[serde(rename = "furry-diffusion-inpainting")]
    FurryDiffusionInpainting,
    #[serde(rename = "kandinsky-vanilla")]
    KandinskyVanilla,
    #[serde(rename = "nai-diffusion-2")]
    NaiDiffusion2,
    #[serde(rename = "nai-diffusion-3")]
    NaiDiffusion3,
}

impl Default for ImageModel {
    fn default() -> Self {
        Self::NaiDiffusion3
    }
}
