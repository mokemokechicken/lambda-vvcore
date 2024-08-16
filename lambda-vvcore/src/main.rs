use base64::{engine::general_purpose, Engine as _};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use lazy_static::lazy_static;
use serde_json::{json, Value};
use std::sync::Mutex;

mod voicevox_wrapper;

use voicevox_wrapper::SimpleVoiceVox;

lazy_static! {
    static ref VOICE_VOX: Mutex<SimpleVoiceVox> = Mutex::new(SimpleVoiceVox::new().unwrap());
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(function_handler)).await?;
    Ok(())
}

fn error_response(message: &str) -> Value {
    eprintln!("{}", message);
    json!({ "error": message })
}

async fn function_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let payload = event.payload;
    println!("Received event: {:?}", payload);

    let input_body: InputBody = if let Some(body) = payload["body"].as_str() {
        // Non-Local environment: Parse JSON string
        match serde_json::from_str(body) {
            Ok(input_body) => input_body,
            Err(e) => {
                return Ok(error_response(&format!("Failed to parse JSON: {}", e)));
            }
        }
    } else {
        // Local environment: Direct InputBody
        match serde_json::from_value(payload) {
            Ok(input_body) => input_body,
            Err(e) => {
                return Ok(error_response(&format!("Failed to parse JSON: {}", e)));
            }
        }
    };

    println!("Received body: {:?}", input_body);

    let text = &input_body.text;
    let speaker_id = input_body.speaker_id.unwrap_or(0);

    if text.is_empty() {
        return Ok(error_response("text is empty"));
    }

    let vv = VOICE_VOX.lock().unwrap();
    match vv.say(text, speaker_id) {
        Ok(wav_data) => {
            let base64_wav = general_purpose::STANDARD.encode(wav_data);
            println!("Generated speech length: {}", base64_wav.len());
            Ok(json!({ "wav": base64_wav }))
        }
        Err(e) => Ok(error_response(&format!("Failed to generate speech: {}", e))),
    }
}

#[derive(Debug, serde::Deserialize)]
struct InputBody {
    text: String,
    speaker_id: Option<u32>,
}
