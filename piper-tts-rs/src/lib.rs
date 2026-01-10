// IMPORTANT NOTE: WORK IN PROGRESS

use piper_tts_rs_sys::piper_audio_chunk;

pub struct PiperAudioChunk {
    pub piper_audio_chunk: piper_audio_chunk
}

impl PiperAudioChunk {
    pub fn new() -> Self {
        let chunk = unsafe { std::mem::MaybeUninit::<piper_audio_chunk>::zeroed().assume_init() };
        Self {
            piper_audio_chunk: chunk
        }
    }
}