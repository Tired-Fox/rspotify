use std::collections::HashMap;

use spotify::spotify_enum::Type;
use super::device::Device;
use super::track::FullTrack;
///https://developer.spotify.com/web-api/get-the-users-currently-playing-track/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    pub uri: String,
    pub href: String,
    pub external_urls: HashMap<String, String>,
    #[serde(rename = "type")]
    pub _type: Type,
}

///https://developer.spotify.com/web-api/get-information-about-the-users-current-playback/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullPlayingContext {
    pub device: Device,
    pub repeat_state: RepeatState,
    pub shuffle_state: bool,
    pub context: Option<Context>,
    pub timestamp: u32,
    pub progress_ms: Option<u32>,
    pub is_playing: bool,
    pub item: Option<FullTrack>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepeatState {
    Off,
    Track,
    Context,
}

///https://developer.spotify.com/web-api/get-the-users-currently-playing-track/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedPlayingContext {
    pub context: Option<Context>,
    pub timestamp: u32,
    pub progress_ms: Option<u32>,
    pub is_playing: bool,
    pub item: Option<FullTrack>,
}