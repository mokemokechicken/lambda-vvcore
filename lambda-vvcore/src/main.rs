use base64::{engine::general_purpose, Engine as _};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use lazy_static::lazy_static;
use serde_json::{json, Value};
use std::env;
use std::sync::Mutex;

mod voicevox_wrapper;

use voicevox_wrapper::SimpleVoiceVox;

lazy_static! {
    static ref VOICE_VOX: Mutex<SimpleVoiceVox> = Mutex::new(SimpleVoiceVox::new().unwrap());
    static ref API_KEY: String = env::var("LAMBDA_APIKEY").expect("LAMBDA_APIKEY must be set");
    static ref LOCAL_MODE: bool = env::var("LOCAL_MODE").is_ok();
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

    // Skip authentication if LOCAL_MODE is set
    if *LOCAL_MODE {
        return process_request(payload).await;
    }

    // Check for Authorization header
    if let Some(headers) = payload["headers"].as_object() {
        if let Some(auth_header) = headers
            .get("Authorization")
            .or(headers.get("authorization"))
        {
            if let Some(token) = auth_header.as_str() {
                if token == format!("Bearer {}", *API_KEY) {
                    // Authentication successful, proceed with the function
                    return process_request(payload).await;
                }
            }
        }
    }

    // If we reach here, authentication failed
    Ok(json!({
        "statusCode": 401,
        "body": json!({ "error": "Unauthorized" }).to_string()
    }))
}

async fn process_request(payload: Value) -> Result<Value, Error> {
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
            Ok(json!({
                "statusCode": 200,
                "body": json!({ "wav": base64_wav }).to_string()
            }))
        }
        Err(e) => Ok(json!({
            "statusCode": 500,
            "body": json!({ "error": format!("Failed to generate speech: {}", e) }).to_string()
        })),
    }
}

#[derive(Debug, serde::Deserialize)]
struct InputBody {
    text: String,
    speaker_id: Option<u32>,
}
