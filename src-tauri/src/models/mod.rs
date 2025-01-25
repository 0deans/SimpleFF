use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoParams {
    pub input_path: String,
    pub output_path: String,
    pub audio_codec: Option<String>,
    pub video_codec: Option<String>,
    #[serde(default)]
    pub video_codec_params: Option<HashMap<String, String>>,
    #[serde(default)]
    pub audio_codec_params: Option<HashMap<String, String>>,
}
