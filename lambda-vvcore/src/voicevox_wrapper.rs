// #![allow(unused_imports, dead_code)]

use std::ffi::CString;
use std::time::Instant;

use vvcore::*;

pub struct SimpleVoiceVox {
    vvc: VoicevoxCore,
}

impl SimpleVoiceVox {
    pub fn new() -> Result<Self, String> {
        // env.OPEN_JTALK_DICT_DIR から辞書ディレクトリを取得
        let dir = std::env::var("OPEN_JTALK_DICT_DIR")
            .map_err(|e| format!("Failed to get OPEN_JTALK_DICT_DIR: {}", e))?;
        let dir_c_str = CString::new(dir.clone())
            .map_err(|e| format!("Failed to convert OPEN_JTALK_DICT_DIR to CString: {}", e))?;
        println!("OPEN_JTALK_DICT_DIR: {}", dir);

        let timer = Instant::now();

        let vvc = VoicevoxCore::new_from_options(AccelerationMode::CPU, 6, false, &dir_c_str) // cpu 6 threads
            .map_err(|e| format!("Failed to initialize VoicevoxCore: {:?}", e))?;

        println!("VoicevoxCore initialized: {:?}", timer.elapsed());

        Ok(Self { vvc })
    }

    pub fn say(&self, text: &str, speaker_id: u32) -> Result<Vec<u8>, String> {
        if !self.vvc.is_model_loaded(speaker_id) {
            // 指定された話者のモデルをロード
            let timer = Instant::now();
            println!("Loading model for speaker {}", speaker_id);
            self.vvc
                .load_model(speaker_id)
                .map_err(|e| format!("Failed to load model for speaker {}: {:?}", speaker_id, e))?;
            println!(
                "Model loaded for speaker {}: {:?}",
                speaker_id,
                timer.elapsed()
            );
        }

        let wav = generate_wav(&self.vvc, text, speaker_id)?;
        Ok(wav)
    }
}

fn generate_wav(vvc: &VoicevoxCore, text: &str, speaker: u32) -> Result<Vec<u8>, String> {
    let timer = Instant::now();
    println!("tts_simple in generate_wav: {}", text);
    let wav: CPointerWrap<u8> = vvc
        .tts(
            text,
            speaker,
            TtsOptions {
                enable_interrogative_upspeak: true,
                kana: false,
            },
        )
        .map_err(|e| format!("Failed to generate WAV: {:?}", e))?;

    println!("tts_simple in generate_wav: {:?}", timer.elapsed());

    // CPointerWrap<u8>からVec<u8>にデータをコピー
    Ok(wav.as_slice().to_vec())
}
